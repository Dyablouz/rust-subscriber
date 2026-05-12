# Reflection

## What is AMQP?

AMQP stands for Advanced Message Queuing Protocol. It is a protocol used by message brokers, such as RabbitMQ, to let applications send and receive messages through queues.

## What does it mean? guest:guest@localhost:5672 , what is the first guest, and what is the second guest, and what is localhost:5672 is for?

In "guest:guest@localhost:5672", the first "guest" is the username used to log in to RabbitMQ. The second "guest" is the password for that username. "localhost:5672" means the RabbitMQ server is running on the local computer, and the application connects to it through port "5672", which is the default AMQP port.

## Simulation Slow Subscriber

![RabbitMQ queue chart](RabbitMQ.png)

In my run, RabbitMQ shows `Queues: 2` in the global counts. The first queue is `user_created`, which is the main queue used by the publisher to send events and by the subscriber to consume them. The second queue is `user_created_dead_letter`, which is created because the subscriber uses `QueueProperties { use_dead_letter: true }`.

The queued messages chart shows a different number. In the screenshot, the total queued messages briefly spikes to around 20-21 messages. This happens because I ran the publisher several times quickly while the subscriber was slowed down with `thread::sleep(ten_millis)`. Each publisher run sends 5 events, so repeated runs can add messages faster than the subscriber can process them. The messages temporarily pile up in the `user_created` queue, then the number goes back down as the subscriber processes them one by one.

## Reflection and Running at least three subscribers

![Multiple subscriber consoles](Multi_Console.png)

![RabbitMQ with three subscribers](RabbitMQ2.png)

After running at least three subscriber consoles, RabbitMQ shows `Consumers: 3`, `Connections: 3`, and `Channels: 3`. This means there are three subscriber processes connected to the same `user_created` queue.

The message queue spike is reduced quicker than before because the work is shared across multiple subscribers. Each subscriber receives different messages from the same queue, so one subscriber may process `Budi` and `Dira`, while another subscriber processes `Amir`, `Cica`, or `Emir`. Since three subscribers process messages in parallel, the queued messages are consumed faster and the RabbitMQ chart returns to zero sooner.

Looking at the code, one improvement is to handle publishing errors instead of ignoring the return value with `_ = p.publish_event(...)`. The publisher should report or retry failed sends. Another improvement is to make the RabbitMQ URL configurable through an environment variable instead of hardcoding `amqp://guest:guest@localhost:5672`, so the same code can run in different environments.