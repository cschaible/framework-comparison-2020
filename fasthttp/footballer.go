package main

import "github.com/francoispqt/gojay"

type Footballer struct {
	Id        int64
	FirstName string
	LastName  string
	Position  string
}

// implement MarshalJSONObject

func (f *Footballer) MarshalJSONObject(enc *gojay.Encoder) {
	enc.Int64Key("id", f.Id)
	enc.StringKey("firstName", f.FirstName)
	enc.StringKey("lastName", f.LastName)
	enc.StringKey("position", f.Position)
}

func (f *Footballer) IsNil() bool {
	return f == nil
}

// implement UnmarshalJSONObject

func (f *Footballer) UnmarshalJSONObject(dec *gojay.Decoder, key string) error {
	switch key {
	case "id":
		return dec.Int64(&f.Id)
	case "firstName":
		return dec.String(&f.FirstName)
	case "lastName":
		return dec.String(&f.LastName)
	case "position":
		return dec.String(&f.Position)
	}
	return nil
}

func (f *Footballer) NKeys() int {
	return 4
}

// define slice type

type Footballers []*Footballer

// implement MarshalJSONArray

func (f *Footballers) MarshalJSONArray(enc *gojay.Encoder) {
	for _, e := range *f {
		enc.Object(e)
	}
}
func (f *Footballers) IsNil() bool {
	return len(*f) == 0
}
