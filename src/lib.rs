use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    get_octo, listen_to_event, octocrab::models::events::payload::WorkflowRunEventAction,
    EventPayload,
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let login = "jaykchen";
    let owner = "jaykchen";
    let repo = "vitesse-lite";

    listen_to_event(&login, &owner, &repo, vec!["workflow_run"], |payload| {
        handler(&login, &owner, &repo, payload)
    })
    .await;
}

async fn handler(login: &str, owner: &str, repo: &str, payload: EventPayload) {
    let octo = get_octo(Some(String::from(login)));
    let issues = octo.issues(owner, repo);
    match payload {
        EventPayload::WorkflowRunEvent(e) => {
            if e.action == WorkflowRunEventAction::Completed {
                let workflow_run = e.workflow_run;
                let conclusion = workflow_run.conclusion.unwrap_or("".to_string());

                if conclusion != String::from("success") {
                    let name = workflow_run.name;
                    let run_number = workflow_run.run_number;

                    let title = format!("{conclusion} executing {name} run #{run_number}");
                    let body = workflow_run.html_url.to_string();

                    match issues.create(title).send().await {
                        Ok(issue) => {
                            let issue_number = issue.number;
                            match issues.create_comment(issue_number, body).await {
                                Ok(_comment) => {}
                                Err(e) => {
                                    write_error_log!(e.to_string());
                                }
                            }
                        }
                        Err(e) => {
                            write_error_log!(e.to_string());
                        }
                    };
                } else {
                    return;
                }
            }
        }
        _ => return,
    };
}
