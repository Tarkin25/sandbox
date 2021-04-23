package com.example.authentication.core;

import org.springframework.data.r2dbc.repository.R2dbcRepository;
import org.springframework.data.repository.NoRepositoryBean;

@NoRepositoryBean
public interface DomainEntityRepository<T extends DomainEntity> extends R2dbcRepository<T, Long> {

}
