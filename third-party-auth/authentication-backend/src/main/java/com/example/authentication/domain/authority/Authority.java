package com.example.authentication.domain.authority;

import com.example.authentication.core.DomainEntity;
import lombok.*;
import lombok.experimental.FieldDefaults;

@NoArgsConstructor
@AllArgsConstructor
@With
@FieldDefaults(level = AccessLevel.PRIVATE)
@Getter
public class Authority extends DomainEntity {

    String name;
}
