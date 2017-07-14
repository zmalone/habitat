---
title: Building a Sample App
---

# Building a Sample App 

Habitat provides a consistent runtime experience irrespective of the environment (containers, bare metal, VMs, etc.). To get that consistent experience, Habitat provides a way to bundle your application and configuration with only the dependencies your app requires. Building packages in Habitat requires two things: the application you want to package, and the plan for how that application should be built and configured. And to streamline the process of creating a plan for your application, Habitat includes default implementations of how to build and configure specific app types such as Node.js and Ruby-on-Rails web applications. These default implementations are called [scaffolding](/docs/concepts-scaffolding) and will be what we use in this tutorial.

The sample application we are packaging is a basic Ruby-on-Rails web application. We will then show how to use that package with a PostgreSQL package to demonstrate the setup of a simple two-tier application.

If you went through the [interactive CLI demo](), this is the tutorial will show you how to build the application that we used in that demo.

**Prerequisites**

Before starting this tutorial, [setup and configure the Habitat CLI]()  on your workstation. Also, we will be using Docker Compose in this tutorial. If you are running this tutorial on a macOS or Windows-based machine and already have Docker for Mac or Docker for Windows installed and running, then you are already setup to use Docker Compose. For Linux users, please follow the [installation instructions](https://docs.docker.com/compose/install/) for Docker Compose.

> Note: The minimum Docker version required for Habitat is greater than or equal to the version specified in the `core/docker` plan, which currently is 1.11.2.

Also, if you are running an older Windows 10 version such as 1511, ANSI escape sequences are not supported. This means the color output and other formatting used by the `hab` CLI will not render properly in your PowerShell window. You can use console emulators like [ConEmu](https://conemu.github.io/) to run PowerShell with ANSI color support.

<hr>
<ul class="main-content--button-nav">
  <li><a href="/tutorials/sample-app/choose-environment" class="button cta">Next - Basic concepts</a></li>
</ul>