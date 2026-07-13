---
kind: Package
id: package:kafka
name: "kafka Knowledge Package"
version: "0.1.0"
purpose: |
  Auto-generated knowledge package crawled from https://kafka.apache.org/documentation/.
  Covers 5 pages of documentation.
problem_solved: |
  Provides structured knowledge extracted from the official kafka.apache.org documentation
  for use in AI agent decision-making.
install: |
  ```bash
  atlas install kafka.md
  ```
concepts:
  - name: "Apache Kafka"
    id: concept:page_0_kafka
    description: |
      Extracted from documentation: Apache Kafka
  - name: "Core capabilities"
    id: concept:page_1_kafka
    description: |
      Extracted from documentation: Core capabilities
  - name: "Trust and Ease of Use"
    id: concept:page_2_kafka
    description: |
      Extracted from documentation: Trust and Ease of Use
  - name: "Getting Started"
    id: concept:page_3_kafka
    description: |
      Extracted from documentation: Getting Started
  - name: "Introduction"
    id: concept:page_4_kafka
    description: |
      Extracted from documentation: Introduction
  - name: "What is event streaming?"
    id: concept:page_5_kafka
    description: |
      Extracted from documentation: What is event streaming?
  - name: "What can I use event streaming for?"
    id: concept:page_6_kafka
    description: |
      Extracted from documentation: What can I use event streaming for?
  - name: "Apache Kafka® is an event streaming platform. What does that mean?"
    id: concept:page_7_kafka
    description: |
      Extracted from documentation: Apache Kafka® is an event streaming platform. What does that mean?
  - name: "How does Kafka work in a nutshell?"
    id: concept:page_8_kafka
    description: |
      Extracted from documentation: How does Kafka work in a nutshell?
  - name: "Main Concepts and Terminology"
    id: concept:page_9_kafka
    description: |
      Extracted from documentation: Main Concepts and Terminology
  - name: "Kafka APIs"
    id: concept:page_10_kafka
    description: |
      Extracted from documentation: Kafka APIs
  - name: "Where to go from here"
    id: concept:page_11_kafka
    description: |
      Extracted from documentation: Where to go from here
  - name: "Quickstart"
    id: concept:page_12_kafka
    description: |
      Extracted from documentation: Quickstart
  - name: "Step 1: Get Kafka"
    id: concept:page_13_kafka
    description: |
      Extracted from documentation: Step 1: Get Kafka
  - name: "Step 2: Start the Kafka environment"
    id: concept:page_14_kafka
    description: |
      Extracted from documentation: Step 2: Start the Kafka environment
  - name: "Step 3: Create a topic to store your events"
    id: concept:page_15_kafka
    description: |
      Extracted from documentation: Step 3: Create a topic to store your events
  - name: "Step 4: Write some events into the topic"
    id: concept:page_16_kafka
    description: |
      Extracted from documentation: Step 4: Write some events into the topic
  - name: "Step 5: Read the events"
    id: concept:page_17_kafka
    description: |
      Extracted from documentation: Step 5: Read the events
  - name: "Step 6: Import/export your data as streams of events with Kafka Connect"
    id: concept:page_18_kafka
    description: |
      Extracted from documentation: Step 6: Import/export your data as streams of events with Kafka Connect
  - name: "Step 7: Process your events with Kafka Streams"
    id: concept:page_19_kafka
    description: |
      Extracted from documentation: Step 7: Process your events with Kafka Streams
  - name: "Step 8: Terminate the Kafka environment"
    id: concept:page_20_kafka
    description: |
      Extracted from documentation: Step 8: Terminate the Kafka environment
  - name: "Congratulations!"
    id: concept:page_21_kafka
    description: |
      Extracted from documentation: Congratulations!
  - name: "Using downloaded files"
    id: concept:page_22_kafka
    description: |
      Extracted from documentation: Using downloaded files
  - name: "Using JVM Based Apache Kafka Docker Image"
    id: concept:page_23_kafka
    description: |
      Extracted from documentation: Using JVM Based Apache Kafka Docker Image
  - name: "Using GraalVM Based Native Apache Kafka Docker Image"
    id: concept:page_24_kafka
    description: |
      Extracted from documentation: Using GraalVM Based Native Apache Kafka Docker Image
apis:
  - name: "$ tar -xzf kafka_2.13-4.3.1.tgz"
    id: api:crawl_0_kafka
    signature: |
      $ tar -xzf kafka_2.13-4.3.1.tgz
      $ cd kafka_2.13-4.3.1
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ KAFKA_CLUSTER_ID=\"$(bin/kafka-storage.sh random-uuid)\""
    id: api:crawl_1_kafka
    signature: |
      $ KAFKA_CLUSTER_ID="$(bin/kafka-storage.sh random-uuid)"
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ bin/kafka-storage.sh format --standalone -t $KAFKA_CLUSTER"
    id: api:crawl_2_kafka
    signature: |
      $ bin/kafka-storage.sh format --standalone -t $KAFKA_CLUSTER_ID -c config/server.properties
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ bin/kafka-server-start.sh config/server.properties"
    id: api:crawl_3_kafka
    signature: |
      $ bin/kafka-server-start.sh config/server.properties
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ docker pull apache/kafka:4.3.1"
    id: api:crawl_4_kafka
    signature: |
      $ docker pull apache/kafka:4.3.1
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ docker run -p 9092:9092 apache/kafka:4.3.1"
    id: api:crawl_5_kafka
    signature: |
      $ docker run -p 9092:9092 apache/kafka:4.3.1
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ docker pull apache/kafka-native:4.3.1"
    id: api:crawl_6_kafka
    signature: |
      $ docker pull apache/kafka-native:4.3.1
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ docker run -p 9092:9092 apache/kafka-native:4.3.1"
    id: api:crawl_7_kafka
    signature: |
      $ docker run -p 9092:9092 apache/kafka-native:4.3.1
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ bin/kafka-topics.sh --create --topic quickstart-events --b"
    id: api:crawl_8_kafka
    signature: |
      $ bin/kafka-topics.sh --create --topic quickstart-events --bootstrap-server localhost:9092
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ bin/kafka-topics.sh --describe --topic quickstart-events -"
    id: api:crawl_9_kafka
    signature: |
      $ bin/kafka-topics.sh --describe --topic quickstart-events --bootstrap-server localhost:9092
      Topic: quickstart-events        TopicId: NPmZHyhbR9y00wMglMH2sg PartitionCount: 1       ReplicationFactor: 1	Configs:
      Topic: quickstart-events Partition: 0    Leader: 0   Replicas: 0 Isr: 0
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ bin/kafka-console-producer.sh --topic quickstart-events --"
    id: api:crawl_10_kafka
    signature: |
      $ bin/kafka-console-producer.sh --topic quickstart-events --bootstrap-server localhost:9092
      >This is my first event
      >This is my second event
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ bin/kafka-console-consumer.sh --topic quickstart-events --"
    id: api:crawl_11_kafka
    signature: |
      $ bin/kafka-console-consumer.sh --topic quickstart-events --from-beginning --bootstrap-server localhost:9092
      This is my first event
      This is my second event
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "connect-file-4.3.1.jar"
    id: api:crawl_12_kafka
    signature: |
      connect-file-4.3.1.jar
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "config/connect-standalone.properties"
    id: api:crawl_13_kafka
    signature: |
      config/connect-standalone.properties
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ echo \"plugin.path=libs/connect-file-4.3.1.jar\" >> config/c"
    id: api:crawl_14_kafka
    signature: |
      $ echo "plugin.path=libs/connect-file-4.3.1.jar" >> config/connect-standalone.properties
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ echo -e \"foo"
    id: api:crawl_15_kafka
    signature: |
      $ echo -e "foo
      bar" > test.txt
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ echo foo > test.txt"
    id: api:crawl_16_kafka
    signature: |
      $ echo foo > test.txt
      $ echo bar >> test.txt
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ bin/connect-standalone.sh config/connect-standalone.proper"
    id: api:crawl_17_kafka
    signature: |
      $ bin/connect-standalone.sh config/connect-standalone.properties config/connect-file-source.properties config/connect-file-sink.properties
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ more test.sink.txt"
    id: api:crawl_18_kafka
    signature: |
      $ more test.sink.txt
      foo
      bar
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ bin/kafka-console-consumer.sh --bootstrap-server localhost"
    id: api:crawl_19_kafka
    signature: |
      $ bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic connect-test --from-beginning
      {"schema":{"type":"string","optional":false},"payload":"foo"}
      {"schema":{"type":"string","optional":false},"payload":"bar"}
      …
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ echo \"Another line\" >> test.txt"
    id: api:crawl_20_kafka
    signature: |
      $ echo "Another line" >> test.txt
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "KStream<String, String> textLines = builder.stream(\"quicksta"
    id: api:crawl_21_kafka
    signature: |
      KStream<String, String> textLines = builder.stream("quickstart-events");
      
      KTable<String, Long> wordCounts = textLines
                  .flatMapValues(line -> Arrays.asList(line.toLowerCase().split(" ")))
                  .groupBy((keyIgnored, word) -> word)
                  .count();
      
      wordCounts.toStream().to("output-topic", Produced.with(Serdes.String(), Serdes.Long()));
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ rm -rf /tmp/kafka-logs /tmp/kraft-combined-logs"
    id: api:crawl_22_kafka
    signature: |
      $ rm -rf /tmp/kafka-logs /tmp/kraft-combined-logs
    returns: See documentation
    description: |
      Extracted code example from documentation.
failures:

---

# kafka

Auto-generated knowledge package crawled from https://kafka.apache.org/documentation/.

**Pages crawled**: 5
**Source**: https://kafka.apache.org/documentation/

# Documentation Redirect | Apache Kafka

Redirecting...

Learn More Download More than 80% of all Fortune 100 companies trust, and use Apache Kafka.
Apache Kafka Apache Kafka is an open-source distributed event streaming platform used by thousands of companies for high-performance data pipelines, streaming analytics, data integration, and mission-critical applications.
Manufacturing 10 OUT OF 10
Banks 7 OUT OF 10
Insurance 10 OUT OF 10
Telecom 8 OUT OF 10
Transportation 8 OUT OF 10
Energy and Utilities 10 OUT OF 10

Above is a snapshot of the number of top-ten largest companies using Kafka, per-industry.See full list

Kafka boasts core capabilities that are battle tested and ready to power businesses in the digital world.

Deliver messages at network limited throughput using a cluster of machines with latencies as low as 2ms.

Scale production clusters up to a thousand brokers, trillions of messages per day, petabytes of data, hundreds of thousands of partitions. Elastically expand and contract storage and processing.

Store streams of data safely in a distributed, durable, fault-tolerant cluster.

Stretch clusters efficiently over availability zones or connect separate clusters across geographic regions.

Process streams of events with joins, aggregations, filters, transformations, and more, using event-time and exactly-once processing.

Kafka’s out-of-the-box Connect interface integrates with hundreds of event sources and event sinks including Postgres, JMS, Elasticsearch, AWS S3, and more.

Kafka is simple to use and is trusted by thousands of organizations around the world.

Support mission-critical use cases with guaranteed ordering, zero message loss, and efficient exactly-once processing.

Thousands of organizations use Kafka, from internet giants to car manufacturers to stock exchanges. More than 5 million unique lifetime downloads.

Kafka is one of the five most active projects of the Apache Software Foundation, with hundreds of meetups around the world.

Read, write and process streams of events in a vast array of programming languages.

Rich documentation, online training, guided tutorials, videos, sample projects, Stack overflow, etc.

Large ecosystem of open source tools: Leverage a vast array of community-driven tooling.

## Apache Kafka

## Core capabilities

## Trust and Ease of Use

# Getting Started | Apache Kafka

This section provides an overview of what Kafka is, why it is useful, and how to get started using it.

## Getting Started

# Introduction | Apache Kafka

What is event streaming? Event streaming is the digital equivalent of the human body’s central nervous system. It is the technological foundation for the ‘always-on’ world where businesses are increasingly software-defined and automated, and where the user of software is more software.
Technically speaking, event streaming is the practice of capturing data in real-time from event sources like databases, sensors, mobile devices, cloud services, and software applications in the form of streams of events; storing these event streams durably for later retrieval; manipulating, processing, and reacting to the event streams in real-time as well as retrospectively; and routing the event streams to different destination technologies as needed.

Technically speaking, event streaming is the practice of capturing data in real-time from event sources like databases, sensors, mobile devices, cloud services, and software applications in the form of streams of events; storing these event streams durably for later retrieval; manipulating, processing, and reacting to the event streams in real-time as well as retrospectively; and routing the event streams to different destination technologies as needed. Event streaming thus ensures a continuous flow and interpretation of data so that the right information is at the right place, at the right time.

Event streaming is applied to a wide variety of use cases across a plethora of industries and organizations. Its many examples include:

To process payments and financial transactions in real-time, such as in stock exchanges, banks, and insurances.

To track and monitor cars, trucks, fleets, and shipments in real-time, such as in logistics and the automotive industry.

To continuously capture and analyze sensor data from IoT devices or other equipment, such as in factories and wind parks.

To collect and immediately react to customer interactions and orders, such as in retail, the hotel and travel industry, and mobile applications.

To monitor patients in hospital care and predict changes in condition to ensure timely treatment in emergencies.

To connect, store, and make available data produced by different divisions of a company.

To serve as the foundation for data platforms, event-driven architectures, and microservices.

Kafka combines three key capabilities so you can implement your use cases for event streaming end-to-end with a single battle-tested solution:

To publish (write) and subscribe to (read) streams of events, including continuous import/export of your data from other systems.

To store streams of events durably and reliably for as long as you want.

To process streams of events as they occur or retrospectively.

And all this functionality is provided in a distributed, highly scalable, elastic, fault-tolerant, and secure manner. Kafka can be deployed on bare-metal hardware, virtual machines, and containers, and on-premises as well as in the cloud. You can choose between self-managing your Kafka environments and using fully managed services offered by a variety of vendors.

Kafka is a distributed system consisting of servers and clients that communicate via a high-performance TCP network protocol. It can be deployed on bare-metal hardware, virtual machines, and containers in on-premise as well as cloud environments.

Servers : Kafka is run as a cluster of one or more servers that can span multiple datacenters or cloud regions. Some of these servers form the storage layer, called the brokers. Other servers run Kafka Connect to continuously import and export data as event streams to integrate Kafka with your existing systems such as relational databases as well as other Kafka clusters. To let you implement mission-critical use cases, a Kafka cluster is highly scalable and fault-tolerant: if any of its servers fails, the other servers will take over their work to ensure continuous operations without any data loss.

Clients : They allow you to write distributed applications and microservices that read, write, and process streams of events in parallel, at scale, and in a fault-tolerant manner even in the case of network problems or machine failures. Kafka ships with some such clients included, which are augmented by dozens of clients provided by the Kafka community: clients are available for Java and Scala including the higher-level Kafka Streams library, for Go, Python, C/C++, and many other programming languages as well as REST APIs.

An event records the fact that “something happened” in the world or in your business. It is also called record or message in the documentation. When you read or write data to Kafka, you do this in the form of events. Conceptually, an event has a key, value, timestamp, and optional metadata headers. Here’s an example event:

Event key: “Alice”

Event value: “Made a payment of $200 to Bob”

Event timestamp: “Jun. 25, 2020 at 2:06 p.m.”

Producers are those client applications that publish (write) events to Kafka, and consumers are those that subscribe to (read and process) these events. In Kafka, producers and consumers are fully decoupled and agnostic of each other, which is a key design element to achieve the high scalability that Kafka is known for. For example, producers never need to wait for consumers. Kafka provides various guarantees such as the ability to process events exactly-once.

Events are organized and durably stored in topics. Very simplified, a topic is similar to a folder in a filesystem, and the events are the files in that folder. An example topic name could be “payments”. Topics in Kafka are always multi-producer and multi-subscriber: a topic can have zero, one, or many producers that write events to it, as well as zero, one, or many consumers that subscribe to these events. Events in a topic can be read as often as needed-unlike traditional messaging systems, events are not deleted after consumption. Instead, you define for how long Kafka should retain your events through a per-topic configuration setting, after which old events will be discarded. Kafka’s performance is effectively constant with respect to data size, so storing data for a long time is perfectly fine.

Topics are partitioned , meaning a topic is spread over a number of “buckets” located on different Kafka brokers. This distributed placement of your data is very important for scalability because it allows client applications to both read and write the data from/to many brokers at the same time. When a new event is published to a topic, it is actually appended to one of the topic’s partitions. Events with the same event key (e.g., a customer or vehicle ID) are written to the same partition, and Kafka guarantees that any consumer of a given topic-partition will always read that partition’s events in exactly the same order as they were written.

Figure: This example topic has four partitions P1-P4. Two different producer clients are publishing, independently from each other, new events to the topic by writing events over the network to the topic’s partitions. Events with the same key (denoted by their color in the figure) are written to the same partition. Note that both producers can write to the same partition if appropriate.

To make your data fault-tolerant and highly-available, every topic can be replicated , even across geo-regions or datacenters, so that there are always multiple brokers that have a copy of the data just in case things go wrong, you want to do maintenance on the brokers, and so on. A common production setting is a replication factor of 3, i.e., there will always be three copies of your data. This replication is performed at the level of topic-partitions.

This primer should be sufficient for an introduction. The Design section of the documentation explains Kafka’s various concepts in full detail, if you are interested.

In addition to command line tooling for management and administration tasks, Kafka has five core APIs for Java and Scala:

The Admin API to manage and inspect topics, brokers, and other Kafka objects.

The Producer API to publish (write) a stream of events to one or more Kafka topics.

The Consumer API to subscribe to (read) one or more topics and to process the stream of events produced to them.

The Kafka Streams API to implement stream processing applications and microservices. It provides higher-level functions to process event streams, including transformations, stateful operations like aggregations and joins, windowing, processing based on event-time, and more. Input is read from one or more topics in order to generate output to one or more topics, effectively transforming the input streams to output streams.

The Kafka Connect API to build and run reusable data import/export connectors that consume (read) or produce (write) streams of events from and to external systems and applications so they can integrate with Kafka. For example, a connector to a relational database like PostgreSQL might capture every change to a set of tables. However, in practice, you typically don’t need to implement your own connectors because the Kafka community already provides hundreds of ready-to-use connectors.

To get hands-on experience with Kafka, follow the Quickstart.

To understand Kafka in more detail, read the Documentation. You also have your choice of Kafka books and academic papers.

Browse through the Use Cases to learn how other users in our world-wide community are getting value out of Kafka.

Join a local Kafka meetup group and watch talks from Kafka Summit, the main conference of the Kafka community.

## Introduction

## What is event streaming?

## What can I use event streaming for?

## Apache Kafka® is an event streaming platform. What does that mean?

## How does Kafka work in a nutshell?

## Main Concepts and Terminology

## Kafka APIs

## Where to go from here

# Quickstart | Apache Kafka

Step 1: Get Kafka Download the latest Kafka release and extract it:
$ tar -xzf kafka_2.13-4.3.1.tgz $ cd kafka_2.13-4.3.1 Step 2: Start the Kafka environment NOTE: Your local environment must have Java 17+ installed.
Kafka can be run using local scripts and downloaded files or the docker image.
Using downloaded files Generate a Cluster UUID
$ KAFKA_CLUSTER_ID="$(bin/kafka-storage.sh random-uuid)" Format Log Directories
$ bin/kafka-storage.sh format --standalone -t $KAFKA_CLUSTER_ID -c config/server.properties Start the Kafka Server

Once the Kafka server has successfully launched, you will have a basic Kafka environment running and ready to use.

Get the Docker image:

Start the Kafka Docker container:

Kafka is a distributed event streaming platform that lets you read, write, store, and process events (also called records or messages in the documentation) across many machines.

Example events are payment transactions, geolocation updates from mobile phones, shipping orders, sensor measurements from IoT devices or medical equipment, and much more. These events are organized and stored in topics. Very simplified, a topic is similar to a folder in a filesystem, and the events are the files in that folder.

So before you can write your first events, you must create a topic. Open another terminal session and run:

All of Kafka’s command line tools have additional options: run the kafka-topics.sh command without any arguments to display usage information. For example, it can also show you details such as the partition count of the new topic:

A Kafka client communicates with the Kafka brokers via the network for writing (or reading) events. Once received, the brokers will store the events in a durable and fault-tolerant manner for as long as you need-even forever.

Run the console producer client to write a few events into your topic. By default, each line you enter will result in a separate event being written to the topic.

You can stop the producer client with Ctrl-C at any time.

Open another terminal session and run the console consumer client to read the events you just created:

You can stop the consumer client with Ctrl-C at any time.

Feel free to experiment: for example, switch back to your producer terminal (previous step) to write additional events, and see how the events immediately show up in your consumer terminal.

Because events are durably stored in Kafka, they can be read as many times and by as many consumers as you want. You can easily verify this by opening yet another terminal session and re-running the previous command again.

You probably have lots of data in existing systems like relational databases or traditional messaging systems, along with many applications that already use these systems. Kafka Connect allows you to continuously ingest data from external systems into Kafka, and vice versa. It is an extensible tool that runs connectors , which implement the custom logic for interacting with an external system. It is thus very easy to integrate existing systems with Kafka. To make this process even easier, there are hundreds of such connectors readily available.

In this quickstart we’ll see how to run Kafka Connect with simple connectors that import data from a file to a Kafka topic and export data from a Kafka topic to a file.

First, make sure to add connect-file-4.3.1.jar to the plugin.path property in the Connect worker’s configuration. For the purpose of this quickstart we’ll use a relative path and consider the connectors’ package as an uber jar, which works when the quickstart commands are run from the installation directory. However, it’s worth noting that for production deployments using absolute paths is always preferable. See plugin.path for a detailed description of how to set this config.

Edit the config/connect-standalone.properties file, add or change the plugin.path configuration property match the following, and save the file:

Then, start by creating some seed data to test with:

Next, we’ll start two connectors running in standalone mode, which means they run in a single, local, dedicated process. We provide three configuration files as parameters. The first is always the configuration for the Kafka Connect process, containing common configuration such as the Kafka brokers to connect to and the serialization format for data. The remaining configuration files each specify a connector to create. These files include a unique connector name, the connector class to instantiate, and any other configuration required by the connector.

These sample configuration files, included with Kafka, use the default local cluster configuration you started earlier and create two connectors: the first is a source connector that reads lines from an input file and produces each to a Kafka topic and the second is a sink connector that reads messages from a Kafka topic and produces each as a line in an output file.

During startup you’ll see a number of log messages, including some indicating that the connectors are being instantiated. Once the Kafka Connect process has started, the source connector should start reading lines from test.txt and producing them to the topic connect-test, and the sink connector should start reading messages from the topic connect-test and write them to the file test.sink.txt. We can verify the data has been delivered through the entire pipeline by examining the contents of the output file:

Note that the data is being stored in the Kafka topic connect-test, so we can also run a console consumer to see the data in the topic (or use custom consumer code to process it):

The connectors continue to process data, so we can add data to the file and see it move through the pipeline:

You should see the line appear in the console consumer output and in the sink file.

Once your data is stored in Kafka as events, you can process the data with the Kafka Streams client library for Java/Scala. It allows you to implement mission-critical real-time applications and microservices, where the input and/or output data is stored in Kafka topics. Kafka Streams combines the simplicity of writing and deploying standard Java and Scala applications on the client side with the benefits of Kafka’s server-side cluster technology to make these applications highly scalable, elastic, fault-tolerant, and distributed. The library supports exactly-once processing, stateful operations and aggregations, windowing, joins, processing based on event-time, and much more.

To give you a first taste, here’s how one would implement the popular WordCount algorithm:

The Kafka Streams demo and the app development tutorial demonstrate how to code and run such a streaming application from start to finish.

Now that you reached the end of the quickstart, feel free to tear down the Kafka environment-or continue playing around.

Stop the producer and consumer clients with Ctrl-C, if you haven’t done so already.

Stop the Kafka broker with Ctrl-C.

If you also want to delete any data of your local Kafka environment including any events you have created along the way, run the command:

You have successfully finished the Apache Kafka quickstart.

To learn more, we suggest the following next steps:

Read through the brief Introduction to learn how Kafka works at a high level, its main concepts, and how it compares to other technologies. To understand Kafka in more detail, head over to the Documentation.

## Quickstart

## Step 1: Get Kafka

## Step 2: Start the Kafka environment

## Step 3: Create a topic to store your events

## Step 4: Write some events into the topic

## Step 5: Read the events

## Step 6: Import/export your data as streams of events with Kafka Connect

## Step 7: Process your events with Kafka Streams

## Step 8: Terminate the Kafka environment

## Congratulations!

## Using downloaded files

## Using JVM Based Apache Kafka Docker Image

## Using GraalVM Based Native Apache Kafka Docker Image

```
$ tar -xzf kafka_2.13-4.3.1.tgz
$ cd kafka_2.13-4.3.1
```

```
$ KAFKA_CLUSTER_ID="$(bin/kafka-storage.sh random-uuid)"
```

```
$ bin/kafka-storage.sh format --standalone -t $KAFKA_CLUSTER_ID -c config/server.properties
```

```
$ bin/kafka-server-start.sh config/server.properties
```

```
$ docker pull apache/kafka:4.3.1
```

```
$ docker run -p 9092:9092 apache/kafka:4.3.1
```

```
$ docker pull apache/kafka-native:4.3.1
```

```
$ docker run -p 9092:9092 apache/kafka-native:4.3.1
```

```
$ bin/kafka-topics.sh --create --topic quickstart-events --bootstrap-server localhost:9092
```

```
$ bin/kafka-topics.sh --describe --topic quickstart-events --bootstrap-server localhost:9092
Topic: quickstart-events        TopicId: NPmZHyhbR9y00wMglMH2sg PartitionCount: 1       ReplicationFactor: 1	Configs:
Topic: quickstart-events Partition: 0    Leader: 0   Replicas: 0 Isr: 0
```

```
$ bin/kafka-console-producer.sh --topic quickstart-events --bootstrap-server localhost:9092
>This is my first event
>This is my second event
```

```
$ bin/kafka-console-consumer.sh --topic quickstart-events --from-beginning --bootstrap-server localhost:9092
This is my first event
This is my second event
```

```
connect-file-4.3.1.jar
```

```
config/connect-standalone.properties
```

```
$ echo "plugin.path=libs/connect-file-4.3.1.jar" >> config/connect-standalone.properties
```

```
$ echo -e "foo
bar" > test.txt
```

```
$ echo foo > test.txt
$ echo bar >> test.txt
```

```
$ bin/connect-standalone.sh config/connect-standalone.properties config/connect-file-source.properties config/connect-file-sink.properties
```

```
$ more test.sink.txt
foo
bar
```

```
$ bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic connect-test --from-beginning
{"schema":{"type":"string","optional":false},"payload":"foo"}
{"schema":{"type":"string","optional":false},"payload":"bar"}
…
```

```
$ echo "Another line" >> test.txt
```

```
KStream<String, String> textLines = builder.stream("quickstart-events");

KTable<String, Long> wordCounts = textLines
            .flatMapValues(line -> Arrays.asList(line.toLowerCase().split(" ")))
            .groupBy((keyIgnored, word) -> word)
            .count();

wordCounts.toStream().to("output-topic", Produced.with(Serdes.String(), Serdes.Long()));
```

```
$ rm -rf /tmp/kafka-logs /tmp/kraft-combined-logs
```


