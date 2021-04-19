package com.example.todo.error;

import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.ResponseStatus;

@ResponseStatus(HttpStatus.NOT_FOUND)
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
public class NotFoundException extends RuntimeException {

    Long id;
    Class<?> entityClass;

    @Override
    public String getMessage() {
        return String.format("%s with id '%s' not found", entityClass.getSimpleName(), id);
    }
}
