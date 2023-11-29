# System Requirements


## Table of Contents

1. [Overview](#overview)
    1. [Purpose and Objectives](#purpose-and-objectives)
    1. [Scope](#scope)
    1. [Target Audience](#target-audience)
1. [Bussiness Requirements](#bussiness-requirements)
1. [Functional Requirements](#functional-requirements)
    1. [User and account](#user-and-account)
    1. [Organization and workspace](#organization-and-workspace)
    1. [Project](#project)
    1. [Task](#task)
1. [Non-Functional Requirements](#non-functional-requirements)


## Overview

### Purpose and Objectives

The purpose of this system is to provide a task management system for users.
The objectives of the system is to improve the efficiency and effectiveness of
task management.

### Scope

The scope of this system includes the following:

- Task creation and management
- Team collaboration and communication

### Target Audience

The system's primary target is development teams that need flexible task
management, but it's a good fit for anyone else.


## Bussiness Requirements

- Task efficiency and effectiveness: The system must help users to create,
  assign, track, and complete tasks efficiently and effectively.
- Employee organization and progress management: The system must help employees
  to organize their tasks, track their progress, and stay on track.


## Functional Requirements

### User and account

- "User" corresponds to the individual who uses the system.
- "User" can have multiple accounts with different profile.
- "Account" is a unit that is linked to various data when users use the system.
- Users can sign in to "User" using email and password, and can use the system
  while switching "Accounts" in that state.

### Organization and workspace

- "Organization" is managed by multiple "Accounts" and forms a team that
  includes multiple "Accounts".
- "Organization" can have multiple "Workspaces".
- "Workspace" is assigned "Accounts" belonging to the "Organization" that
  manages it.

### Project

- "Workspace" can manage multiple "Projects".
- "Project" can be assigned "Accounts" that exist in "Workspace".

### Task

- "Project" can manage multiple "Tasks".
- "Task" can be assigned "Accounts" belonging to the "Project".
- "Task" can have deadlines and status management.


## Non-Functional Requirements

- The system should be easily scalable and have no single point of failure.
