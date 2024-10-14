#![allow(non_snake_case)]

use diamond_types::list::{Branch as ListBranch, OpLog as ListOpLog};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level}; // OpLog と Branch を正しくインポート

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut oplog = use_signal(ListOpLog::new); // OpLog のインスタンスを作成
    let mut input_value = use_signal(String::new);

    // TODO: ログイン機能を実装する
    // ログインユーザのidをセット
    let agent = oplog.write().get_or_create_agent_id("ryotarofr"); // AgentId を取得

    oplog.write().add_insert(agent, 0, "abc123"); // "abc123" を挿入
    oplog.write().add_delete_without_content(agent, 1..2); // 'b' を削除
    oplog.write().add_insert(agent, 0, "ababab"); // "ababab" を挿入

    // Branch のインスタンスを作成し、OpLog の内容を反映
    // // 現在のリストの内容を取得
    let branch_content = oplog.with(|log| {
        let branch = ListBranch::new_at_tip(log); // Branch のインスタンスを作成
        branch.content().to_string()
    });

    // 結果を出力
    info!("{:?}", oplog);
    info!("{}", branch_content.to_string());
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        textarea {
            value: "{input_value}",
            // oninput: move |e| {
            //     let new_value = e.value().clone();
            //     if *input_value.read() != new_value {
            //         input_value.set(new_value.clone());
            //         oplog.write().add_insert(agent, 0, &new_value);  // 入力値をoplogに追加
            //     }
            // }
        }
    }
}
