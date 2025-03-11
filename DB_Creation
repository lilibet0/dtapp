//*
    References https://www.akc.org/sports/obedience/getting-started/classes/ for exercise descriptions
*//

// Table Creations
Table 1 Query:
CREATE TABLE Obedience_Class
(class_name varchar(20) not null,
points int,
title varchar(3),
primary key(class_name));

Table 2 Query:
CREATE TABLE Prerequisite_Class 
(class_name varchar(20), 
prerequisite_class varchar(20),
primary key(class_name),
foreign key(class_name) references Obedience_Class(class_name),
foreign key(prerequisite_class) references Obedience_Class(class_name));

Table 3 Query:
CREATE TABLE Exercise
(exercise_name varchar(50),
exercise_description varchar(400),
possible_points int,
class_name varchar(20),
primary key(exercise_name),
foreign key(class_name) references Obedience_Class(class_name));


// Table Insertions
Table 1 Insert:
// class name, total possible points, title abbreviation
INSERT into Obedience_Class VALUES
('Novice', 200, 'CD'),
('Open', 200, 'CDX'),
('Utility', 200, 'UD');

Table 2 Insert:
// class name, prereq class name
INSERT into Prerequisite_Class VALUES
('Novice', null),
('Open', 'Novice'),
('Utility', 'Open');

Table 3 Insert:
// exercise name, description, possible points
INSERT INTO Exercise VALUES
('Heel on Leash and Figure Eight', 'show whether the dog has learned to watch its handler and adjust its pace to stay with the handler.', 40, 'Novice'),
('Stand for Examination', 'for this off-leash exercise the dog must stay in a standing position as its handler walks about 6 feet away. The judge will then lightly touch the dog on the head, body and hindquarters.', 30, 'Novice'),
('Heel Free', 'dogs will heel off-leash doing the same routine as they did on-leash, except they will not perform the Figure Eight.', 40, 'Novice'),
('Recall', 'demonstrates that the dog will come to the handler on command.', 30, 'Novice'),
('Sit Stay – Get Your Leash', 'demonstrate that the dog will remain in the sit position, while the handler goes to get the dog’s leash.', 30, 'Novice'),
('Group Exercise – Sit & Down Stay', 'this exercise will demonstrate the dog’s ability to remain in the sit and down position, with other dogs in the ring and is performed on-leash.', 30, 'Novice'),
('Heel Free and Figure Eight', 'this exercise is performed the same as Novice, but the dog is off-leash.', 40, 'Open'),
('Command Discrimination (Stand, Down, Sit)', 'handlers will be instructed by the judge to stand, down or sit their dog from varying distances. Handlers will be instructed by the judge to have their dog change positions three times. The second and third position change are with the handler 15 and 30 feet from the dog.', 30, 'Open'),
('Drop on Recall', 'the dog must promptly come to the handler when called from across the ring and on the handler’s command or signal to drop and remain in a down position until on a command or signal from the handler to resume coming to the handler.', 30, 'Open'),
('Retrieve on Flat', 'demonstrates a dog’s ability to retrieve an object from at least 20 feet promptly and return to the handler on command.', 20, 'Open'),
('Retrieve Over High Jump', 'the dog must go out over a jump, in order to retrieve a dumbbell and then promptly return to the handler with the dumbbell going back over the jump.', 30, 'Open'),
('Broad Jump', 'this exercise shows that the dog will stay in the position it is left until directed to jump. The dog must clear the jump on a single command or signal and return to the handler once the jump is complete.', 20, 'Open'),
('Stand Stay – Get Your Leash', 'the principal feature of this exercise is that the dog stand and stay in position until the handler has returned to heel position.', 30, 'Open'),
('Signal Exercise', 'shows the dog’s ability to understand and correctly respond to the handler’s signal to stand, stay, down, sit and come. No voice commands are given; only hand signals are allowed.', 40, 'Utility'),
('Scent Discrimination', 'this exercise is done twice. Showing the dog’s ability to find the handler’s scent among a pile of articles and promptly return the correct article to the handler.', 60, 'Utility'),
('Directed Retrieve', 'the features of this exercise prove the dog’s ability to follow a directional signal from the handler to retrieve a glove and promptly return it to the handler.', 30, 'Utility'),
('Moving Stand and Examination', 'for this exercise the dog must heel, stand and stay as the handler moves away. The dog must stay and accept an examination by the judge and return to the handler on command.', 30, 'Utility'),
('Directed Jumping', 'the dog must go away from the handler, turn and sit. Then, the dog must clear whichever jump its handler indicates and promptly return to the handler.', 40, 'Utility');
