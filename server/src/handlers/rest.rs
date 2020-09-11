use super::db;
use crate::{
    models::{HttpVerb, NewRecipe, NewRule, RecipeCascaded, Rule, RuleType},
    DbPool,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    http::Uri,
    web::{self, Bytes, Data, Path},
    HttpResponse, Result,
};
use anyhow::{bail, format_err, Context};
use serde_json::Value;
use std::convert::TryInto;
use uuid::Uuid;

#[actix_web::get("/api/v1/recipe")]
pub(crate) async fn list_recipes(db_pool: Data<DbPool>) -> Result<HttpResponse> {
    let json = web::block(move || {
        super::list_recipes_offset_limit(db_pool, super::DEFAULT_OFFSET, super::DEFAULT_LIMIT)
    })
    .await
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json))
}

#[actix_web::get("/api/v1/recipe/offset/{offset}")]
pub(crate) async fn list_recipes_page(
    db_pool: Data<DbPool>,
    offset: Path<i64>,
) -> Result<HttpResponse> {
    let json = web::block(move || {
        super::list_recipes_offset_limit(db_pool, offset.into_inner(), super::DEFAULT_LIMIT)
    })
    .await
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json))
}

#[actix_web::get("/api/v1/recipe/{id}")]
pub(crate) async fn get_recipe(path: Path<Uuid>, db: Data<DbPool>) -> Result<HttpResponse> {
    let (recipe, rules) = web::block(move || db::find_recipe(&db, path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)?;
    let body: shared::Recipe = RecipeCascaded(recipe, rules)
        .try_into()
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(body))
}

#[actix_web::post("/api/v1/recipe")]
pub(crate) async fn create_recipe(db_pool: Data<DbPool>, recipe: Bytes) -> Result<HttpResponse> {
    let recipe: Value = serde_json::from_slice(&recipe)
        .with_context(|| "Could not parse the post body as JSON!")
        .map_err(ErrorBadRequest)?;
    let shared::Recipe {
        url,
        payload,
        rules,
        ..
    } = validate_post(recipe).map_err(ErrorBadRequest)?;
    let payload = serde_json::to_string(&payload).map_err(ErrorInternalServerError)?;
    let (recipe, rules) = {
        let to_create = NewRecipe { url, payload };
        web::block(move || {
            db::create_recipe(&db_pool, to_create).and_then(|recipe| {
                let to_create: Vec<NewRule> = rules
                    .into_iter()
                    .map(|rule| (recipe.id, rule).into())
                    .collect();
                db::create_rules(&db_pool, &to_create).map(|rules| (recipe, rules))
            })
        })
        .await
        .map_err(ErrorInternalServerError)?
    };
    let created: shared::Recipe = RecipeCascaded(recipe, rules)
        .try_into()
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::put("/api/v1/recipe")]
pub(crate) async fn update_recipe(db_pool: Data<DbPool>, recipe: Bytes) -> Result<HttpResponse> {
    let recipe: Value = serde_json::from_slice(&recipe)
        .with_context(|| "Could not parse the post body as JSON!")
        .map_err(ErrorBadRequest)?;
    let shared::Recipe {
        id,
        url,
        payload,
        rules,
        ..
    } = validate_put(recipe).map_err(ErrorBadRequest)?;
    let id = id
        .ok_or_else(|| format_err!("Must specify Id when udpating a recipe!"))
        .map_err(ErrorBadRequest)?;
    let payload = serde_json::to_string(&payload)?;
    let (recipe, rules) = {
        use shared::Rule::*;
        web::block(move || {
            let count = db::update_recipe(&db_pool, id, url, payload)?;
            if count == 1 {
                let (to_retain, to_create): (Vec<shared::Rule>, Vec<shared::Rule>) =
                    rules.into_iter().partition(|rule| match rule {
                        Authenticated { id, .. } | Subject { id, .. } | HttpMethod { id, .. } => {
                            id.is_some()
                        }
                    });
                let to_retain = to_retain
                    .into_iter()
                    .map(|rule| (id, rule).try_into())
                    .collect::<anyhow::Result<Vec<Rule>>>()?;
                let to_create = to_create
                    .into_iter()
                    .map(|rule| (id, rule).into())
                    .collect::<Vec<NewRule>>();
                db::delete_rules(&db_pool, id, &to_retain)?;
                db::update_rules(&db_pool, to_retain)?;
                db::create_rules(&db_pool, &to_create)?;
                db::find_recipe(&db_pool, id)
            } else {
                bail!("Unable to update recipe, {}", id)
            }
        })
        .await
        .map_err(ErrorInternalServerError)?
    };
    let created: shared::Recipe = RecipeCascaded(recipe, rules)
        .try_into()
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::delete("/api/v1/recipe/{id}")]
pub(crate) async fn delete_recipe(db_pool: Data<DbPool>, path: Path<Uuid>) -> Result<HttpResponse> {
    let to_delete = path.into_inner();
    web::block(move || db::delete_recipe(&db_pool, to_delete))
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(format!("Deleted recipe, {}", to_delete)))
}

fn validate_post(post: Value) -> anyhow::Result<shared::Recipe> {
    validate_change(post, "create")
}

fn validate_put(put: Value) -> anyhow::Result<shared::Recipe> {
    put.get("id")
        .ok_or_else(|| format_err!("You must include an ID with a new recipe!"))?;
    validate_change(put, "update")
}

fn validate_change(value: Value, action: &str) -> anyhow::Result<shared::Recipe> {
    let endpoint = value
        .get("url")
        .and_then(Value::as_str)
        .ok_or_else(|| format_err!("You must specify a URL in order to {} a recipe!", action))?;
    validate_url(endpoint)?;
    if let Some(rules) = value.get("rules") {
        let rules = rules
            .as_array()
            .ok_or_else(|| format_err!("Rules property must be an array of JSON objects!"))?;
        validate_rules(&rules)?;
    }
    value.get("payload").ok_or_else(|| {
        format_err!(
            "You must include a payload in order to {} a recipe!",
            action
        )
    })?;
    serde_json::from_value(value).map_err(anyhow::Error::from)
}

fn validate_url(endpoint: &str) -> anyhow::Result<()> {
    let endpoint = endpoint
        .parse::<Uri>()
        .with_context(|| format_err!("Could not parse the URL field, {}, as a URL!", endpoint))?;
    if endpoint.scheme().is_none() {
        bail!("The URL for a recipe has to include a scheme!")
    }
    if endpoint.host().is_none() {
        bail!("The URL for a recipe has to include a host!")
    }
    if !endpoint
        .path_and_query()
        .map(|path_and_query| path_and_query.as_str().starts_with("/api"))
        .unwrap_or_default()
    {
        bail!("The path of the URL must start with \"/api\"!")
    }
    Ok(())
}

fn validate_rules(rules: &[Value]) -> anyhow::Result<()> {
    rules
        .iter()
        .map(validate_rule)
        .collect::<anyhow::Result<()>>()?;
    Ok(())
}

fn validate_rule(rule: &Value) -> anyhow::Result<()> {
    let rule = rule
        .as_object()
        .ok_or_else(|| format_err!("Rule must be a JSON object!"))?;
    if rule.len() != 1 {
        bail!("Rule JSON can only have one property, whose name must match a rule type, e.g. \"Authenticated\", \"Subject\", or \"HttpMethod\"")
    }
    let rule_type = rule
        .keys()
        .next()
        .ok_or_else(|| format_err!("Rule element should have a single property!"))?;
    let rule = rule.get(rule_type).ok_or_else(|| {
        format_err!(
            "Rule property, {}, should have a valid JSON value!",
            rule_type
        )
    })?;
    let rule_type: RuleType = rule_type.as_str().try_into()?;

    use RuleType::*;
    match rule_type {
        Authenticated => validate_authenticated(&rule),
        Subject => validate_subject(&rule),
        HttpMethod => validate_http_method(&rule),
    }
}

fn validate_authenticated(rule: &Value) -> anyhow::Result<()> {
    rule.get("key_path").and_then(Value::as_str).ok_or_else(|| format_err!("The rule type, \"Authenticated\", must have a property, \"key_path\", in its body with a string value!"))?;
    Ok(())
}

fn validate_subject(rule: &Value) -> anyhow::Result<()> {
    rule.get("subject").and_then(Value::as_str).ok_or_else(|| format_err!("The rule type, \"Subject\", must have a property, \"subject\", in its body with a string value!"))?;
    Ok(())
}

fn validate_http_method(rule: &Value) -> anyhow::Result<()> {
    let http_verb = rule.get("http_method").ok_or_else(|| {
        format_err!("The rule type, \"HttpMethod\", must have a property, \"http_method\"!")
    })?;
    let http_verb = http_verb.as_str().ok_or_else(|| {
        format_err!("The value of the property, \"http_method\", must be a string!")
    })?;
    let _http_verb: HttpVerb = http_verb.try_into()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validation() -> anyhow::Result<()> {
        let json = json! {{
            "url": "http://test.local/api/rest",
            "rules": [
                {
                    "Authenticated":{"key_path":"foo"}
                },
                {
                    "Subject":{"subject":"example"}
                },
                {
                    "HttpMethod":{"http_method":"Get"}
                }
            ],
            "payload": {
                "foo": "bar"
            }
        }};

        validate_post(json)?;
        Ok(())
    }

    #[test]
    fn test_empty_rules() -> anyhow::Result<()> {
        let json = json! {{
            "url": "http://test.local/api/rest",
            "rules": [],
            "payload": {
                "foo": "bar"
            }
        }};

        validate_post(json)?;
        Ok(())
    }

    #[test]
    fn test_no_rules() -> anyhow::Result<()> {
        let json = json! {{
            "url": "http://test.local/api/rest",
            "payload": {
                "foo": "bar"
            }
        }};

        validate_post(json)?;
        Ok(())
    }

    #[test]
    fn test_missing_payload() -> anyhow::Result<()> {
        let json = json! {{
            "url": "http://test.local/api/rest",
            "rules": []
        }};

        if let Err(error) = validate_post(json) {
            assert!(
                error.to_string().contains("payload"),
                "Error should have been about missing payload!"
            );
            Ok(())
        } else {
            bail!("Validation should failed due to missing payload!")
        }
    }

    #[test]
    fn test_invalid_scheme() -> anyhow::Result<()> {
        if let Err(error) = validate_url("//test.local/api/rest") {
            assert!(
                error.to_string().contains("scheme"),
                "Error should have been about invalid scheme!"
            );
            Ok(())
        } else {
            bail!("Validation should have failed due to invalid scheme")
        }
    }

    #[test]
    fn test_invalid_path() -> anyhow::Result<()> {
        if let Err(error) = validate_url("http://test.local/rest") {
            assert!(
                error.to_string().contains("path"),
                "Error should have been about invalid path!"
            );
            Ok(())
        } else {
            bail!("Validation should have failed due to invalid path")
        }
    }

    #[test]
    fn test_invalid_rule_type() -> anyhow::Result<()> {
        if let Err(error) = validate_rule(&json! {{
            "InvalidType":{}
        }}) {
            assert!(
                error.to_string().contains("rule type"),
                "Error should have been about invalid rule type!"
            );
            Ok(())
        } else {
            bail!("Validation should have failed due to invalid rule type")
        }
    }

    #[test]
    fn test_auth_no_keypath() -> anyhow::Result<()> {
        if let Err(error) = validate_authenticated(&json! {{
            "foo":"bar"
        }}) {
            assert!(
                error.to_string().contains("key_path"),
                "Error should have been about missing key path! ({})",
                error
            );
            Ok(())
        } else {
            bail!("Validation should have failed due to missing key path")
        }
    }

    #[test]
    fn test_subject_no_subject() -> anyhow::Result<()> {
        if let Err(error) = validate_subject(&json! {{
            "foo":"bar"
        }}) {
            assert!(
                error.to_string().contains("subject"),
                "Error should have been about missing subject! ({})",
                error
            );
            Ok(())
        } else {
            bail!("Validation should have failed due to missing subject")
        }
    }

    #[test]
    fn test_method_no_verb() -> anyhow::Result<()> {
        if let Err(error) = validate_http_method(&json! {{
            "foo":"bar"
        }}) {
            assert!(
                error.to_string().contains("http_method"),
                "Error should have been about missing http method! ({})",
                error
            );
            Ok(())
        } else {
            bail!("Validation should have failed due to missing http method")
        }
    }

    #[test]
    fn test_method_invalid_verb() -> anyhow::Result<()> {
        if let Err(error) = validate_http_method(&json! {{
            "http_method":"foo"
        }}) {
            assert!(
                error.to_string().contains("HTTP verb"),
                "Error should have been about invalid http method! ({})",
                error
            );
            Ok(())
        } else {
            bail!("Validation should have failed due to invalid http method")
        }
    }
}
