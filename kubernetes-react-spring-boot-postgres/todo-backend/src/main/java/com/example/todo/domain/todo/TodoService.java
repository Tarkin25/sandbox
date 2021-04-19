package com.example.todo.domain.todo;

import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

public interface TodoService {

    Flux<Todo> findAll();

    Mono<Todo> findById(Long id);

    Mono<Todo> create(Todo todo);

    Mono<Todo> completeById(Long id);

    Mono<Void> deleteById(Long id);

}
