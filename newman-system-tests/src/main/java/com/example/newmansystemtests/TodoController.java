package com.example.newmansystemtests;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Collections;
import java.util.List;

@RestController
@RequestMapping("/todos")
public class TodoController {

    @GetMapping
    public List<Object> findAll() {
        return Collections.emptyList();
    }

}
