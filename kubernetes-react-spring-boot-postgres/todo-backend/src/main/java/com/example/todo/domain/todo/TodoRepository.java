package com.example.todo.domain.todo;

import org.springframework.data.r2dbc.repository.R2dbcRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface TodoRepository extends R2dbcRepository<Todo, Long> {
}
