#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use mishaarok::app::App;
use mishaarok::models::_entities::sea_orm_active_enums::TunnelsState;
use mishaarok::models::_entities::{tunnels, users};

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let _ctx = playground::<App>().await?;

    // let active_model: articles::ActiveModel = ActiveModel {
    //     title: Set(Some("how to build apps in 3 steps".to_string())),
    //     content: Set(Some("use Loco: https://loco.rs".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);
    println!("welcome to playground. edit me at `examples/playground.rs`");

    let user = users::ActiveModel {
        ..Default::default()
    };

    let tunnel_active_model = tunnels::ActiveModel {
        created_at: Default::default(),
        updated_at: Default::default(),
        id: Default::default(),
        url: Default::default(),
        user_id: Default::default(),
        state: ActiveValue::Set(TunnelsState::Active),
    };

    Ok(())
}
