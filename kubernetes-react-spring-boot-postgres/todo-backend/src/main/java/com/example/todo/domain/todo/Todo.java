package com.example.todo.domain.todo;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.With;
import lombok.experimental.FieldDefaults;
import org.springframework.data.annotation.Id;

@Data
@FieldDefaults(level = AccessLevel.PRIVATE)
@AllArgsConstructor
@With
public class Todo {

    @Id
    Long id;

    String title;

    String description;

    Boolean completed;

}
