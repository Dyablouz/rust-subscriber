# Rust Subscriber

## What is AMQP?

AMQP stands for Advanced Message Queuing Protocol. It is a protocol used by message brokers, such as RabbitMQ, to let applications send and receive messages through queues.

## What does `guest:guest@localhost:5672` mean?

In `amqp://guest:guest@localhost:5672`, the first `guest` is the username used to log in to RabbitMQ. The second `guest` is the password for that username. `localhost:5672` means the RabbitMQ server is running on the local computer, and the application connects to it through port `5672`, which is the default AMQP port.
