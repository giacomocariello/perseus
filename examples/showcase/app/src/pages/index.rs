use perseus::{StringResultWithCause, Template};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPageProps {
    pub greeting: String,
}

#[component(IndexPage<G>)]
pub fn index_page(props: IndexPageProps) -> SycamoreTemplate<G> {
    template! {
        p {(props.greeting)}
        a(href = "/about") { "About!" }
    }
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("index")
        .build_state_fn(Arc::new(get_static_props))
        .template(template_fn())
}

pub async fn get_static_props(_path: String) -> StringResultWithCause<String> {
    Ok(serde_json::to_string(&IndexPageProps {
        greeting: "Hello World!".to_string(),
    })
    .unwrap())
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Arc::new(|props, _| {
        template! {
            IndexPage(
                serde_json::from_str::<IndexPageProps>(&props.unwrap()).unwrap()
            )
        }
    })
}
