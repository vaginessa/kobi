use crate::database::download::{download_comic, DOWNLOAD_DATABASE};
use crate::database::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::{Expr, IntoColumnRef, SimpleExpr};
use sea_orm::EntityTrait;
use sea_orm::{ConnectionTrait, DeleteResult, QuerySelect};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryInto;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "download_comic_chapter")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub comic_path_word: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: String,
    pub comic_id: String,
    pub count: i64,
    pub datetime_created: String,
    pub group_path_word: String,
    pub img_type: i64,
    pub index: i64,
    pub is_long: bool,
    pub name: String,
    pub news: String,
    pub next: Option<String>,
    pub ordered: i64,
    pub prev: Option<String>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    //
    pub download_status: i64,
    // pub contents: Vec<ChapterImage>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
    if !index_exists(
        db.deref(),
        "download_comic_chapter",
        "download_comic_chapter_idx_comic_path_word",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_chapter",
            vec!["comic_path_word"],
            "download_comic_chapter_idx_comic_path_word",
        )
        .await;
    }
}
