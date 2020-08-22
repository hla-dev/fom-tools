# 2. Structure with Cargo workspace

Date: 2020-08-22

## Status

Accepted

## Context

It is envisioned that this project will create a number of projects all working
off the same underlying representations.

## Decision

We will structure this project as a Cargo workspace.

## Consequences

A cargo workspace will allow a library crate and a number of binary crates to be
published.
