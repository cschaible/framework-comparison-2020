package info.novatec;

import io.r2dbc.pool.ConnectionPool;
import io.r2dbc.pool.ConnectionPoolConfiguration;
import io.r2dbc.postgresql.PostgresqlConnectionConfiguration;
import io.r2dbc.postgresql.PostgresqlConnectionFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.autoconfigure.jdbc.DataSourceProperties;
import org.springframework.boot.jdbc.DataSourceBuilder;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Primary;
import org.springframework.data.r2dbc.connectionfactory.R2dbcTransactionManager;
import org.springframework.data.r2dbc.core.DatabaseClient;
import org.springframework.transaction.ReactiveTransactionManager;
import org.springframework.transaction.annotation.EnableTransactionManagement;
import org.springframework.transaction.reactive.TransactionalOperator;
import org.springframework.web.util.UriComponents;
import org.springframework.web.util.UriComponentsBuilder;

import javax.sql.DataSource;

@Configuration
@EnableTransactionManagement
public class R2dbcConfig {

  @Value("${spring.datasource.initial-pool-size}")
  private int initialPoolSize;

  @Value("${spring.datasource.maximum-pool-size}")
  private int maximumPoolSize;

  @Autowired private DataSourceProperties dataSourceProperties;

  @Bean
  public PostgresqlConnectionFactory connectionFactory() {
    String jdbcUrl = dataSourceProperties.getUrl();
    String shortenedJdbcUrl =
        jdbcUrl.length() > 5 && jdbcUrl.startsWith("jdbc:") ? jdbcUrl.substring(5) : jdbcUrl;
    UriComponents uriComponents = UriComponentsBuilder.fromUriString(shortenedJdbcUrl).build();

    return new PostgresqlConnectionFactory(
        PostgresqlConnectionConfiguration.builder()
            .host(uriComponents.getHost())
            .port(uriComponents.getPort() == -1 ? 5432 : uriComponents.getPort())
            .database(uriComponents.getPath().substring(1))
            .username(uriComponents.getQueryParams().getFirst("user"))
            .password(uriComponents.getQueryParams().getFirst("password"))
            .build());
  }

  @Bean
  public ConnectionPool connectionPool(PostgresqlConnectionFactory connectionFactory) {
    return new ConnectionPool(
        ConnectionPoolConfiguration.builder(connectionFactory)
            .initialSize(initialPoolSize)
            .maxSize(maximumPoolSize)
            .build());
  }

  @Bean
  public DatabaseClient databaseClient(ConnectionPool connectionPool) {
    return DatabaseClient.create(connectionPool);
  }

  @Bean
  @Primary
  public ReactiveTransactionManager transactionManager(
      PostgresqlConnectionFactory connectionFactory) {
    return new R2dbcTransactionManager(connectionFactory);
  }

  /**
   * Configure data source bean (required for flyway)
   *
   * @return the data source bean
   */
  @Bean
  public DataSource dataSource() {
    DataSourceBuilder<?> dataSourceBuilder = DataSourceBuilder.create();
    dataSourceBuilder.driverClassName(dataSourceProperties.getDriverClassName());
    dataSourceBuilder.url(dataSourceProperties.getUrl());
    return dataSourceBuilder.build();
  }
}
