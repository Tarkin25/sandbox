package com.example.gcppubsubspring;

import com.google.cloud.spring.pubsub.PubSubAdmin;
import com.google.cloud.spring.pubsub.core.PubSubTemplate;
import com.google.cloud.spring.pubsub.core.subscriber.PubSubSubscriberTemplate;
import com.google.cloud.spring.pubsub.reactive.PubSubReactiveFactory;
import com.google.cloud.spring.pubsub.support.AcknowledgeablePubsubMessage;
import com.google.pubsub.v1.Subscription;
import com.google.pubsub.v1.Topic;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;
import reactor.core.publisher.Flux;

@SpringBootApplication
@Slf4j
public class GcpPubsubSpringApplication {

    public static void main(String[] args) {
        SpringApplication.run(GcpPubsubSpringApplication.class, args);
    }

    @Bean
    public CommandLineRunner createTopic(PubSubAdmin admin, PubSubReactiveFactory reactiveFactory, PubSubTemplate template) {
        return args -> {
            String topicName = "test-topic";
            String subscriptionName = "test-subscription";

            Topic topic = admin.listTopics()
                    .stream()
                    .filter(t -> t.getName().contains(topicName))
                    .findFirst()
                    .orElseGet(() -> admin.createTopic(topicName));

            Subscription subscription = admin.listSubscriptions()
                    .stream()
                    .filter(s -> s.getName().contains(subscriptionName))
                    .findFirst()
                    .orElseGet(() -> admin.createSubscription(subscriptionName, topic.getName()));

            var messages = reactiveFactory.poll(subscription.getName(), 500L);

            var future = messages
                    .doOnNext(message -> {
                        log.info("Received message: {}", message.getPubsubMessage().getData().toStringUtf8());
                    })
                    .doOnNext(AcknowledgeablePubsubMessage::ack)
                    .collectList()
                    .toFuture();

            template.publish(topic.getName(), "This is a test message");

            future.get();
        };
    }

}
