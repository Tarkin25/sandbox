package com.example.authentication.domain.role;

import com.example.authentication.core.DomainEntity;
import lombok.*;
import lombok.experimental.FieldDefaults;

@NoArgsConstructor
@AllArgsConstructor
@With
@FieldDefaults(level = AccessLevel.PRIVATE)
@Getter
public class Role extends DomainEntity {

    String name;

}
