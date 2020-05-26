package main

import (
	"bytes"
	"database/sql"
	"github.com/francoispqt/gojay"
	"github.com/go-pg/pg/v9"
	"github.com/valyala/fasthttp"
	"strconv"
	"strings"
)

type FootballerController struct {
	footballerRepository FootballerRepository
}

func (c FootballerController) search(ctx *fasthttp.RequestCtx) {
	position := string(ctx.QueryArgs().Peek("Position"))
	var footballers *Footballers
	var err error
	if len(position) == 0 {
		footballers, err = c.footballerRepository.findAll()
	} else {
		footballers, err = c.footballerRepository.findByPosition(position)
	}
	handleErrorOr(ctx, err, func() {
		json, err := c.toJsonFootballers(footballers)
		c.buildJsonResponse(ctx, json, err, 200)
	})
}

func (c FootballerController) get(ctx *fasthttp.RequestCtx) {
	var id, err = strconv.ParseInt(ctx.UserValue("Id").(string), 10, 64)
	handleErrorOr(ctx, err, func() {
		footballer, err := c.footballerRepository.findById(id)
		handleErrorOr(ctx, err, func() {
			json, err := c.toJsonFootballer(footballer)
			c.buildJsonResponse(ctx, json, err, 200)
		})
	})
}

func (c FootballerController) create(ctx *fasthttp.RequestCtx) {
	footballer, err := c.toFootballer(ctx.PostBody())
	handleErrorOr(ctx, err, func() {
		footballer, err = c.footballerRepository.create(*footballer)
		handleErrorOr(ctx, err, func() {
			json, err := c.toJsonFootballer(footballer)
			c.buildJsonResponse(ctx, json, err, 201)
		})
	})
}

func (c FootballerController) delete(ctx *fasthttp.RequestCtx) {
	var id, err = strconv.ParseInt(ctx.UserValue("Id").(string), 10, 64)
	handleErrorOr(ctx, err, func() {
		err = c.footballerRepository.deleteById(id)
		handleErrorOr(ctx, err, func() {
			ctx.SetStatusCode(204)
		})
	})
}

func (c FootballerController) toJsonFootballer(footballer *Footballer) (string, error) {
	b := strings.Builder{}
	enc := gojay.NewEncoder(&b)
	if err := enc.Encode(footballer); err != nil {
		return "", err
	}
	return b.String(), nil
}

func (c FootballerController) toJsonFootballers(footballers *Footballers) (string, error) {
	b := strings.Builder{}
	enc := gojay.NewEncoder(&b)
	if err := enc.EncodeArray(footballers); err != nil {
		return "", err
	}
	return b.String(), nil
}

func (c FootballerController) toFootballer(rawUser []byte) (*Footballer, error) {
	footballer := &Footballer{}
	dec := gojay.NewDecoder(bytes.NewReader(rawUser))
	defer dec.Release()
	if err := dec.DecodeObject(footballer); err != nil {
		return nil, err
	}
	return footballer, nil
}

func (c FootballerController) buildJsonResponse(ctx *fasthttp.RequestCtx, json string, err error, status int) {
	handleErrorOr(ctx, err, func() {
		ctx.SetBody([]byte(json))
		ctx.SetStatusCode(status)
		ctx.SetContentType("application/json")
	})
}

func handleErrorOr(ctx *fasthttp.RequestCtx, err error, runner func()){
	if err != nil {
		if sql.ErrNoRows == err || pg.ErrNoRows == err {
			ctx.SetStatusCode(404)
		} else {
			ctx.SetStatusCode(500)
		}
		ctx.SetBodyString(err.Error())
		return
	} else {
		runner()
	}
}
