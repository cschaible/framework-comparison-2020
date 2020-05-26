package main

import (
	"github.com/go-pg/pg/v9"
)

type FootballerRepository struct {
	db *pg.DB
}

func (repo FootballerRepository) create(footballer Footballer) (*Footballer, error) {
	err := repo.db.Insert(&footballer)
	if err != nil {
		return nil, err
	}
	return &footballer, nil
}

func (repo FootballerRepository) findByPosition(position string) (*Footballers, error) {
	var footballers Footballers
	err := repo.db.Model(&footballers).Where("Position = ?", position).Select()
	if err != nil {
		return nil, err
	}
	return &footballers, nil
}

func (repo FootballerRepository) findById(id int64) (*Footballer, error) {
	footballer := &Footballer{Id: id}
	err := repo.db.Select(footballer)
	if err != nil {
		return nil, err
	}
	return footballer, nil
}

func (repo FootballerRepository) findAll() (*Footballers, error) {
	var footballers Footballers
	err := repo.db.Model(&footballers).Select()
	if err != nil {
		return nil, err
	}
	return &footballers, nil
}

func (repo FootballerRepository) deleteById(id int64)  error {
	footballer, err := repo.findById(id)
	if err != nil {
		return err
	}
	if footballer != nil {
		return repo.db.Delete(footballer)
	}
	return nil
}
