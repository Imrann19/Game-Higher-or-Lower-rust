use rand::Rng;//Cette ligne importe la bibliotheque du random/This line imports the random library
use std::io;//Cette ligne importe la bibliotheque du io(Input output)/This line imports the io(Input output) library

fn main(){//Cette ligne cree la fonction main/This line creates the main function
    let mut rng = rand::thread_rng();//Cette ligne permet de cree une variable rng pour le random/This line allows to create an rng variable for random
    let nb = rng.gen_range(1..=100);//Cette ligne permet de stocker un nombre aleatoire entre 1 et 100 dans la variable nb(immuable)/This line allows to store a random number between 1 and 100 in the variable nb (immutable)
    loop{//Cette ligne permet de cree une boucle/This line allows to create a loop
        println!("Entrer un nombre entre 1 et 100");//Cette ligne permet d'afficher ce message "Entrer un nombre entre 1 et 100"/This line displays the message "Enter a number between 1 and 100"
        let mut input = String::new();//Cette ligne permet de cree une nouvelle chaine de caractère dans la variable input/This line allows to create a new character string in the input variable
        io::stdin() //Cette ligne permet de lire les entrees de l'utilisateur./This line is used to read user input
            .read_line(&mut input)//Cette ligne permet de stocker la saisie de l'utilisateur dans la variable input/This line stores the user's input in the input variable
            .expect("Erreur");//Cette ligne permet d'afficher ce message "Erreur" si jamais il y a une erreur avec le input/This line allows to display this "Error" message if there is ever an error with the input
        let input:i64 = input.trim().parse().expect("Ce n'est pas un nombre");//Cette ligne permet d'abord de supprimer les espaces et de convertir la saisie de l'utilisateur en un nombre(si jamais l'utilisateur a ecrit un texte il y aura une erreur et l'ordinateur va nous afficher ce message "Ce n'est pas un nombre")/This line first removes spaces and converts the user's input into a number (if the user ever writes a text there will be an error and the computer will display this message "This is not a number")
        if input == nb{//Cette ligne permet de cree une condition 'if'(si la variable input est egale a la variable nb alors tout les fonctions dans le bloc 'if' vont etre executee)/This line allows to create an 'if' condition (if the input variable is equal to the nb variable then all the functions in the 'if' block will be executed)
            println!("Vous avez gagner le nombre est {}", nb);//Cette ligne permet d'afficher ce message "Vous avez gagner le nombre est {valeur de nb}"/This line allows to display this message "You have won the number is {value of nb}"
            break;//Cette ligne permet de stopper la boucle/This line allows to stop the loop
        } else{//Cette ligne permet d'executer les fonctions qui sont dans le bloc 'else' si la condition 'if input == nb' ne remplie pas tout les criteres/This line allows to execute the functions that are in the 'else' block if the condition 'if input == nb' does not meet all the criteria
            if input > nb{//Cette ligne permet de cree une condition 'if'(si la variable input est superieur a la variable nb alors tout les fonctions dans le bloc 'if' vont etre executee)/This line allows to create an 'if' condition (if the input variable is greater than the nb variable then all the functions in the 'if' block will be executed)
                println!("Le nombre est plus petit");//Cette ligne permet d'afficher ce message "Le nombre est plus petit"/This line allows to display this message "The number is smaller"
            } else{//Cette ligne permet d'executer les fonctions qui sont dans le bloc 'else' si la condition 'if input > nb' ne remplie pas tout les criteres/This line allows to execute the functions that are in the 'else' block if the condition 'if input > nb' does not meet all the criteria
                println!("Le  nombre est plus grand");//Cette ligne permet d'afficher ce message "Le nombre est plus grand"/This line allows to display this message "The number is bigger"
            }
        }
    }
}
