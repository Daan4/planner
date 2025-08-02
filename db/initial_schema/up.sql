CREATE TABLE `tasks`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`title` TEXT NOT NULL,
	`important` BOOLEAN NOT NULL,
	`urgent` BOOLEAN NOT NULL,
	`content` TEXT,
	`completed` BOOLEAN NOT NULL,
    `role_id` TEXT,
	`backlog_id` TEXT,
	`scheduled_date` DATE,
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP,
	`deleted_at` TIMESTAMP,
	FOREIGN KEY(role_id) REFERENCES roles(id),
	FOREIGN KEY(backlog_id) REFERENCES backlogs(id)
);

CREATE TABLE `backlogs`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL
);

CREATE TABLE `roles`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL
);