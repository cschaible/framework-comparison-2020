import {MigrationInterface, QueryRunner} from "typeorm";

export class CreateFootballerTable1592511865851 implements MigrationInterface {

    public async up(queryRunner: QueryRunner): Promise<void> {
        await queryRunner.query("create table footballer(" +
            "id BIGSERIAL PRIMARY KEY, " +
            "first_name VARCHAR, " +
            "last_name VARCHAR, " +
            "position VARCHAR);")
    }

    public async down(queryRunner: QueryRunner): Promise<void> {
        await queryRunner.query("drop table footballer;")
    }

}
