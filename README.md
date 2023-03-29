# A GitHub App that files an GitHub issue when user's workflow action fails


[Deploy this function on flows.network](#deploy-this-bot-on-your-github-repo), and you will get a GitHub app that creating a GitHub issue automatically when the github workflow action fails. It helps maintainers and PR creators to track issue. The title of this issue is the name of your GitHub Action workflows.

![image](https://user-images.githubusercontent.com/45785633/228459461-54173859-cf78-42f1-87e3-f4b0eceb2be7.png)


## Deploy this bot on your GitHub repo

To install the GitHub Actions Reminder GitHub App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/create-github-issue-workflow-fails/) and customize the code based on your needs. 

<img width="624" alt="image" src="https://user-images.githubusercontent.com/45785633/228460183-53536d8b-b376-4c81-916d-1d10bde3ac93.png">

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying the App
3. Authenticate the [flows.network](https://flows.network/) to access the `create-github-issue-workflow-fails` repo you just forked. 

<img width="889" alt="image" src="https://user-images.githubusercontent.com/45785633/228461712-82f2cecd-4d7c-4b68-b8dd-98e7221fffad.png">


4. click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow. Here we can see, we need to configue one SaaS integration.

<img width="912" alt="image" src="https://user-images.githubusercontent.com/45785633/228461855-026080fb-1aec-4633-80f1-b2190a926794.png">


Click the "Connect/+ Add new authentication" button to authenticate your GitHub account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a repo. This repo is the one you entered into the code above.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the GitHub Action Alert App goes live. Go ahead and when the action fails, you will get a GitHub issue!


> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!








