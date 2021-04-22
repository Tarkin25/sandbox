package com.example.todo.domain.todo;

import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

@RestController
@RequestMapping("/todos")
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
public class TodoController {

    TodoService todoService;

    @GetMapping
    public Flux<Todo> findAll() {
        return todoService.findAll();
    }

    @GetMapping("/{id}")
    public Mono<Todo> findById(@PathVariable Long id) {
        return todoService.findById(id);
    }

    @PostMapping
    @ResponseStatus(HttpStatus.CREATED)
    public Mono<Todo> create(@RequestBody Todo todo) {
        return todoService.create(todo);
    }

    @PutMapping("/{id}")
    public Mono<Todo> completeById(@PathVariable Long id, @RequestBody Todo todo) {
        return todoService.updateById(id, todo);
    }

    @DeleteMapping("/{id}")
    public Mono<Void> deleteById(@PathVariable Long id) {
        return todoService.deleteById(id);
    }

}
