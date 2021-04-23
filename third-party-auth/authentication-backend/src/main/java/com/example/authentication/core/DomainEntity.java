package com.example.authentication.core;

import lombok.*;
import lombok.experimental.FieldDefaults;
import org.springframework.data.annotation.Id;

@NoArgsConstructor
@AllArgsConstructor
@With
@FieldDefaults(level = AccessLevel.PRIVATE)
@Getter
public class DomainEntity {

    @Id
    Long id;

}
