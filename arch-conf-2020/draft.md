# Arch Conf Infrastructure Talk

**Infrastructure at Arch - Making servers go brrrrr**

## Abstract

The number of services Arch provides has steadily grown over the years. How many services and servers are there, and how does it even all fit together?

Also, what do we have planned for the future and how can you help?

Find out in this talk!

## Description

Arch Linux is a complex open-source project with many moving parts, quite a few pieces of infrastructure, and many people involved. Arch uses some modern DevOps tools like Ansible and Terraform.
All of our infrastructure is entirely in the open (except for where is unreasonable to do so).
While we do try to follow the general Arch dogma of keeping things simple, this sometimes conflicts with
doing things _right_ and maintainable.

We're mostly hosted at Hetzner with a few sponsored servers for non-critical and non-trusted services.

Among our public services are:

- Accounts/SSO
- AUR
- BBS
- Bug tracker
- GitLab
- Mailing lists
- Main page and package list
- Mirror list
- Patchwork
- Security tracker
- Wiki

Next to these public services, we also operate quite a few staff-only services such as:

- Build servers
- Kanboard
- Mail servers
- Monitoring stack: Grafana, Prometheus, Alertmanager
- Phrik
- Quassel

Our servers and services used to be hand-configured without any audit trail. In the past years, we've started
the effort of formalising all services we operate using Ansible and Terraform which has been very helpful.

### Ongoing efforts

While there have always been minor efforts here and there for various tasks, in recent times we've seen some
increased activity especially in these topics:

- Automation: Many parts in Arch have always been done manually. We're currently trying to automate many aspects
  of the release process for the ISO, VMs and Docker (all separate projects with different requirements).
- Keycloak SSO migration: We want to have a central account service that allows for secure user account  and access management.
- GitLab migration: We want our GitLab to be the central point for code contribution, issue reporting, and automation.
  A full migration would also hopefully allow us to get rid of Flyspray, Kanboard, and Patchwork which decreases stack complexity.
- Increase user engagement: As Arch Linux is a 100% volunteer-driven project, we need to be as friendly and
  welcoming to any potential outside contributors as we can. To this end, we're trying to increase bus factors
  and visibility to get users engaged.

### The future

We have a long list things we eventually want to have and we're slowly moving there. The general outlook is:

- More contributors
- More automation
- More openness
- More granular access
