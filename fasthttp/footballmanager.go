package main

import (
	"context"
	"flag"
	"fmt"
	"github.com/fasthttp/router"
	"github.com/go-pg/migrations/v7"
	"github.com/go-pg/pg/v9"
	"github.com/joho/godotenv"
	"github.com/valyala/fasthttp"
	"log"
	"os"
)

const usageText = `This program runs command on the db. Supported commands are:
  - init - creates version info table in the database
  - up - runs all available migrations.
  - up [target] - runs available migrations up to the target one.
  - down - reverts last migration.
  - reset - reverts all migrations.
  - version - prints current db version.
  - set_version [version] - sets db version without running migrations.
Usage:
  go run *.go <command> [args]
`

func main() {
	flag.Usage = usage
	flag.Parse()

	env := os.Getenv("APP_ENV")
	if "" == env {
		env = "local"
	}

	err := godotenv.Load(".env." + env)
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	database := pg.Connect(&pg.Options{
		User:     os.Getenv("DB_USER"),
		PoolSize: 15,
		Addr:     os.Getenv("DB_ADDR"),
		Password: os.Getenv("DB_PASSWORD"),
		Database: os.Getenv("DB_NAME"),
	})
	defer database.Close()

	// Log db-statements
	if os.Getenv("LOG_DB_STATEMENTS") == "true" {
		database.AddQueryHook(dbLogger{})
	}

	if len(flag.Args()) == 0 {
		migrate(database, []string{"init"})
		migrate(database, []string{"up"})
	} else {
		migrate(database, flag.Args())
		return
	}

	repo := &FootballerRepository{db: database}

	controller := &FootballerController{
		footballerRepository: *repo,
	}

	r := router.New()
	r.GET("/footballers", controller.search)
	r.GET("/footballers/{Id}", controller.get)
	r.POST("/footballers", controller.create)
	r.DELETE("/footballers/{Id}", controller.delete)

	server := &fasthttp.Server{
		Handler:      r.Handler,
		ErrorHandler: errorHandler,
	}

	log.Fatal(server.ListenAndServe(":8080"))
}

func migrate(database *pg.DB, args []string) {
	oldVersion, newVersion, err := migrations.Run(database, args...)
	if err != nil {
		exit(err.Error())
	}
	if newVersion != oldVersion {
		fmt.Printf("Migrated from version %d to %d\n", oldVersion, newVersion)
	} else {
		fmt.Printf("Skip already applied migration version %d\n", oldVersion)
	}
}

func errorHandler(ctx *fasthttp.RequestCtx, err error) {
	ctx.SetStatusCode(400)
	ctx.SetBodyString(err.Error())
}

func usage() {
	fmt.Print(usageText)
	flag.PrintDefaults()
	os.Exit(2)
}

func exit(s string, args ...interface{}) {
	fmt.Fprintf(os.Stderr, s+"\n", args...)
	os.Exit(1)
}

type dbLogger struct{}

func (d dbLogger) BeforeQuery(c context.Context, q *pg.QueryEvent) (context.Context, error) {
	return c, nil
}

func (d dbLogger) AfterQuery(c context.Context, q *pg.QueryEvent) error {
	fmt.Println(q.FormattedQuery())
	return nil
}
