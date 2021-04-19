package com.example.todo.domain.todo;

import com.example.todo.error.NotFoundException;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

@Service
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
public class TodoServiceImpl implements TodoService {

    TodoRepository repository;

    @Override
    public Flux<Todo> findAll() {
        return repository.findAll();
    }

    @Override
    public Mono<Todo> findById(Long id) {
        return repository.findById(id)
                .switchIfEmpty(notFound(id));
    }

    @Override
    public Mono<Todo> create(Todo todo) {
        return repository.save(todo.withId(null).withCompleted(false));
    }

    @Override
    public Mono<Todo> completeById(Long id) {
        return findById(id)
                .map(todo -> todo.withCompleted(true))
                .flatMap(repository::save);
    }

    @Override
    public Mono<Void> deleteById(Long id) {
        return findById(id)
                .flatMap(repository::delete);
    }

    private <T> Mono<T> notFound(Long id) {
        return Mono.error(new NotFoundException(id, Todo.class));
    }
}
