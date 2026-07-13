---
kind: Package
id: package:grpc
name: "grpc Knowledge Package"
version: "0.1.0"
purpose: |
  Auto-generated knowledge package crawled from https://grpc.io/docs/.
  Covers 5 pages of documentation.
problem_solved: |
  Provides structured knowledge extracted from the official grpc.io documentation
  for use in AI agent decision-making.
install: |
  ```bash
  atlas install grpc.md
  ```
concepts:
  - name: "Documentation"
    id: concept:page_0_grpc
    description: |
      Extracted from documentation: Documentation
  - name: "Official support"
    id: concept:page_1_grpc
    description: |
      Extracted from documentation: Official support
  - name: "A high performance, open source universal RPC framework"
    id: concept:page_2_grpc
    description: |
      Extracted from documentation: A high performance, open source universal RPC framework
  - name: "Why gRPC?"
    id: concept:page_3_grpc
    description: |
      Extracted from documentation: Why gRPC?
  - name: "Used by"
    id: concept:page_4_grpc
    description: |
      Extracted from documentation: Used by
  - name: "Simple service definition"
    id: concept:page_5_grpc
    description: |
      Extracted from documentation: Simple service definition
  - name: "Start quickly and scale"
    id: concept:page_6_grpc
    description: |
      Extracted from documentation: Start quickly and scale
  - name: "Works across languages and platforms"
    id: concept:page_7_grpc
    description: |
      Extracted from documentation: Works across languages and platforms
  - name: "Bi-directional streaming and integrated auth"
    id: concept:page_8_grpc
    description: |
      Extracted from documentation: Bi-directional streaming and integrated auth
  - name: "About gRPC"
    id: concept:page_9_grpc
    description: |
      Extracted from documentation: About gRPC
  - name: "Who’s using gRPC and why?"
    id: concept:page_10_grpc
    description: |
      Extracted from documentation: Who’s using gRPC and why?
  - name: "The story behind gRPC"
    id: concept:page_11_grpc
    description: |
      Extracted from documentation: The story behind gRPC
  - name: "Who is using gRPC and why"
    id: concept:page_12_grpc
    description: |
      Extracted from documentation: Who is using gRPC and why
  - name: "The main usage scenarios"
    id: concept:page_13_grpc
    description: |
      Extracted from documentation: The main usage scenarios
  - name: "Core features that make it awesome"
    id: concept:page_14_grpc
    description: |
      Extracted from documentation: Core features that make it awesome
  - name: "Guides"
    id: concept:page_15_grpc
    description: |
      Extracted from documentation: Guides
apis:

failures:

---

# grpc

Auto-generated knowledge package crawled from https://grpc.io/docs/.

**Pages crawled**: 5
**Source**: https://grpc.io/docs/

# Documentation | gRPC

Learn about key gRPC concepts, try a quick start, find tutorials and reference
material for all supported languages and platforms:


New to gRPC? …

A high-performance, open source universal RPC framework

New to gRPC? Start with the following pages:Introduction to gRPCCore concepts, architecture and lifecycleFAQ

Eager to see gRPC in action?Select a language or platform, then choose its Quick start.

Interested in gRPC feature details?Try one of the following:Select a language or platform, then choose Tutorial or API referenceGuides

These are the officially supported gRPC language, platform and OS versions:

See https://opensource.google/documentation/policies/cplusplus-support

Java 8+ (KitKat+ for Android)

## Documentation

## Official support

A high performance, open source universal RPC …

gRPC is a modern open source high performance Remote Procedure Call (RPC)
framework that can run in any environment. It can efficiently connect services
in and across data centers with pluggable support for load balancing, tracing,
health checking and authentication. It is also applicable in last mile of
distributed computing to connect devices, mobile applications and browsers to
backend services.

Define your service using Protocol Buffers, a powerful binary serialization toolset and language

Install runtime and dev environments with a single line and also scale to millions of RPCs per second with the framework

Automatically generate idiomatic client and server stubs for your service in a variety of languages and platforms

Bi-directional streaming and fully integrated pluggable authentication with HTTP/2-based transport

## A high performance, open source universal RPC framework

## Why gRPC?

## Used by

## Simple service definition

## Start quickly and scale

## Works across languages and platforms

## Bi-directional streaming and integrated auth

# About gRPC | gRPC

Who is using gRPC and why

The main usage scenariosCore features that make it awesome

Who’s using gRPC and why?

The story behind gRPC

Efficiently connecting polyglot services in microservices style architecture

Connecting mobile devices, browser clients to backend services

Generating efficient client libraries

Idiomatic client libraries in 11 languages

Highly efficient on wire and with a simple service definition framework

Bi-directional streaming with http/2 based transport

Pluggable auth, tracing, load balancing and health checking

Many companies are already using gRPC for connecting multiple services in their
environments. The use case varies from connecting a handful of services to
hundreds of services across various languages in on-prem or cloud environments.
Below are details and quotes from some of our early adopters.

Check out what people are saying below.

At Square, we have been collaborating with Google so that we can replace all uses of our custom RPC solution to use gRPC. We decided to move to gRPC because of its open support for multiple platforms, the demonstrated performance of the protocol, and the ability to customize and adapt it to our network. Developers at Square are looking forward to being able to take advantage of writing streaming APIs and in the future, push gRPC to the edges of the network for integration with mobile clients and third party APIs.

In our initial use of gRPC we’ve been able to extend it easily to live within our opinionated ecosystem. Further, we’ve had great success making improvements directly to gRPC through pull requests and interactions with Google’s team that manages the project. We expect to see many improvements to developer productivity, and the ability to allow development in non-JVM languages as a result of adopting gRPC.

Our switch from a home-grown RPC system to gRPC was seamless. We quickly took advantage of the per-stream flow control to provide better scheduling of large RPCs over the same connection as small ones.

With support for high performance bi-directional streaming, TLS based security, and a wide variety of programming languages, gRPC is an ideal unified transport protocol for model driven configuration and telemetry.

The fact that gRPC is built on HTTP/2 transport brings us native bi-directional streaming capabilities and flexible custom-metadata in request headers. The first point is important for large payload-exchange and network-telemetry scenarios, while the latter enables us to expand and include capabilities including but not limited to various network element authentication mechanisms.
In addition, the wide language binding support that gRPC/proto3 brings, enables us to provide a flexible and rapid development environment for both internal and external consumers.
Last but not least, while there are a number of network communication protocols for configuration, operational state retrieval and network telemetry, gRPC provides us with a unified flexible protocol and transport to ease client/server interaction.

gRPC was initially created by Google, which has used a single general-purpose
RPC infrastructure called Stubby to connect the large number of microservices
running within and across its data centers for over a decade. In March 2015,
Google decided to build the next version of Stubby and make it open source. The
result was gRPC, which is now used in many organizations outside of
Google to power use cases from microservices to the “last mile” of computing
(mobile, web, and Internet of Things).

For more background on why we created gRPC, see the gRPC Motivation and Design
Principles on the gRPC blog.

Our table of officially supported languages and platforms has moved!
See Official support.

## About gRPC

## Who’s using gRPC and why?

## The story behind gRPC

## Who is using gRPC and why

## The main usage scenarios

## Core features that make it awesome

# Guides | gRPC

Task-oriented walkthroughs of common use cases

The documentation covers the following techniques:

An overview of gRPC authentication, including built-in auth mechanisms, and how to plug in your own authentication systems.

gRPC is designed to support high-performance open-source RPCs in many languages. This page describes performance benchmarking tools, scenarios considered by tests, and the testing infrastructure.

Explains how and when to cancel RPCs.

How to compress the data sent over the wire while using gRPC.

A mechanism in the gRPC library that allows users to inject custom metrics at the gRPC server and consume at gRPC clients to make your custom load balancing algorithms.

Explains how custom load balancing policies can help optimize load balancing under unique circumstances.

Explains standard name resolution, the custom name resolver interface, and how to write an implementation.

Explains how deadlines can be used to effectively deal with unreliable backends.

Explains the debugging process of gRPC applications using grpcdebug

How gRPC deals with errors, and gRPC error codes.

Explains what flow control is and how you can manually control it.

Explains how to gracefully shut down a gRPC server to avoid causing RPC failures for connected clients.

Explains how gRPC servers expose a health checking service and how client can be configured to automatically check the health of the server it is connecting to.

Explains how interceptors can be used for implementing generic behavior that applies to many RPC methods.

How to use HTTP/2 PING-based keepalives in gRPC.

Explains what metadata is, how it is transmitted, and what it is used for.

OpenTelemetry Metrics available in gRPC

A user guide of both general and language-specific best practices to improve performance.

Explains how reflection can be used to improve the transparency and interpretability of RPCs.

Explains what request hedging is and how you can configure it.

gRPC takes the stress out of failures! Get fine-grained retry control and detailed insights with OpenCensus and OpenTelemetry support.

How the service config can be used by service owners to control client behavior.

Explains the status codes used in gRPC.

Explains how to configure RPCs to wait for the server to be ready before sending the request.

## Guides


