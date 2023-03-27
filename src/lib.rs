use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::models::events::payload::WorkflowRunEventAction,
    EventPayload::{self, WorkflowRunEvent},
};
use slack_flows::send_message_to_channel;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let login = "jaykchen";
    let owner = "jaykchen";
    let repo = "a-test";

    listen_to_event(&login, &owner, &repo, vec!["workflow_run", ""], |payload| {
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
                let workflow = e.workflow;
                let state = workflow.state;

                if state == String::from("failure") || state == String::from("error") {
                    let id = workflow.id.to_string().parse::<u64>().unwrap();
                    let title = workflow.name;
                    let html_url = workflow.html_url;
                    let body = format!("Workflow: {title} {state}\n @{html_url} \n");
                send_message_to_channel("ik8", "ch_in", html_url.to_string());


                    match issues.create(body).send().await {
                        Ok(comment) => {
                            send_message_to_channel("ik8", "ch_out", comment.body_text.unwrap());
                        }
                        Err(e) => {
                            write_error_log!(e.to_string());
                            send_message_to_channel("ik8", "ch_err", e.to_string());
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
