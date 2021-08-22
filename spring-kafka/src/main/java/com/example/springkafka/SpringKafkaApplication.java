package com.example.springkafka;

import org.springframework.boot.ApplicationRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;
import org.springframework.kafka.core.KafkaTemplate;

@SpringBootApplication
public class SpringKafkaApplication {

    public static void main(String[] args) {
        SpringApplication.run(SpringKafkaApplication.class, args);
    }

    @Bean
    public ApplicationRunner sendMessageOnStartup(KafkaTemplate<String, String> template) {
        return args -> {
          template.send("test_topic", "Hello World!");
        };
    }

}
