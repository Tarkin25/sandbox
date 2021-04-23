package com.example.authentication.domain.userrole;

import com.example.authentication.core.DomainEntity;
import lombok.*;
import lombok.experimental.FieldDefaults;

@NoArgsConstructor
@AllArgsConstructor
@With
@FieldDefaults(level = AccessLevel.PRIVATE)
@Getter
public class UserRole extends DomainEntity {

    Long userId;

    Long roleId;

}
