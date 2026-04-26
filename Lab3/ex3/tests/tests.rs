
#[cfg(test)]
mod test {
    use ex3::solution::CircularBuffer;
    use ex3::solution::CircularBufferError;
    use ex3::solution::TryDeref;

    #[test]
    fn test_write_size() {
        let mut cb = CircularBuffer::new(5);

        match cb.write(1) {
            Ok(_) => {
                assert_eq!(cb.size(), 1);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_write_read() {
        let mut cb = CircularBuffer::new(5);

        match cb.write(1) {
            Ok(_) => {
                match cb.read() {
                    Some(result) => assert_eq!(result, 1),
                    None => println!("No val readed"),
                }
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_sequence() {
        let mut cb = CircularBuffer::new(5);

        for i in 0..3 {
            match cb.write(i) {
                Ok(_) => { 
                    println!("{} inserted", i); 
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            }
        }

        for i in 0..3 {
            match cb.read() {
                Some(result) => assert_eq!(result, i),
                None => println!("No val readed"),
            }
        }
    }

    #[test]
    fn test_wrap_around() {
        let capacity = 5;
        let mut cb = CircularBuffer::new(capacity);

        for i in 0..capacity {
            match cb.write(i) {
                Ok(_) => { 
                    println!("{} inserted", i); 
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            }
        }

        for _ in 0..capacity {
            match cb.read() {
                Some(result) => println!("Readed: {}", result),
                None => println!("No val readed"),
            }
        }

        assert_eq!(cb.head, 0);
        assert_eq!(cb.tail, 0);

        for i in 0..capacity {
            match cb.write(i) {
                Ok(_) => { 
                    println!("{} inserted", i); 
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            }
        }

        cb.clear();
        assert_eq!(cb.head, 0);
        assert_eq!(cb.tail, 0);
    }

    #[test]
    fn test_read_empty() {
        let capacity = 5;
        let mut cb: CircularBuffer<i32> = CircularBuffer::new(capacity);

        match cb.read() {
            Some(_result) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_write_full() {
        let capacity = 1;
        let mut cb = CircularBuffer::new(capacity);

        match cb.write(1) {
            Ok(_) => { 
                println!("{} inserted", 1); 
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }

        match cb.write(2) {
            Ok(_) => { 
                assert!(false);
            }
            Err(_err) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_overwrite() {
        let capacity = 3;
        let mut cb = CircularBuffer::new(3);

        for i in 0..capacity {
            match cb.write(i) {
                Ok(_) => { 
                    println!("{} inserted", i); 
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            }
        }

        for _ in 0..capacity {
            match cb.read() {
                Some(result) => println!("Readed: {}", result),
                None => println!("No val readed"),
            }
        }

        cb.overwrite(4);
        match cb.read() {
            Some(result) => assert_eq!(result, 4),
            None => assert!(false),
        }
    }

    #[test]
    fn test_make_contiguous() {
        let mut cb = CircularBuffer::new(5);

        cb.write('A').unwrap();
        cb.write('B').unwrap();
        cb.write('C').unwrap();

        assert_eq!(cb.read(), Some('A'));
        assert_eq!(cb.read(), Some('B'));

        cb.write('D').unwrap();
        cb.write('E').unwrap();
        cb.write('F').unwrap();

        assert_eq!(cb.head, 2);
        assert_eq!(cb.tail, 1);

        cb.make_contiguous();

        assert_eq!(cb.head, 0, "La testa deve essere tornata all'indice 0");
        assert_eq!(cb.tail, 4, "La coda deve essere all'indice 4 (count % capacity)");

        assert_eq!(cb.read(), Some('C'));
        assert_eq!(cb.read(), Some('D'));
    }

    #[test]
    fn test_indexing_standard() {
        let mut cb = CircularBuffer::new(5);
        cb.write(10).unwrap();
        cb.write(20).unwrap();
        cb.write(30).unwrap();

        // Verifichiamo che l'accesso con [] restituisca i valori giusti
        assert_eq!(cb[0], 10);
        assert_eq!(cb[1], 20);
        assert_eq!(cb[2], 30);
    }

    #[test]
    fn test_indexing_with_wrap_around() {
        let mut cb = CircularBuffer::new(5);
        
        // Riempiamo un po' il buffer
        cb.write(10).unwrap();
        cb.write(20).unwrap();
        cb.write(30).unwrap();

        // Leggiamo due elementi per far avanzare la 'head' all'indice fisico 2
        cb.read(); // Toglie il 10
        cb.read(); // Toglie il 20

        // Scriviamo altri tre elementi. 
        // Questo farà sì che il 60 finisca all'indice fisico 0 (wrap-around).
        cb.write(40).unwrap();
        cb.write(50).unwrap();
        cb.write(60).unwrap();

        // Logicamente, il buffer per l'utente è [30, 40, 50, 60]
        // Verifichiamo che il calcolo matematico trovi i valori giusti ovunque siano
        assert_eq!(cb[0], 30); // Fisicamente all'indice 2
        assert_eq!(cb[1], 40); // Fisicamente all'indice 3
        assert_eq!(cb[2], 50); // Fisicamente all'indice 4
        assert_eq!(cb[3], 60); // Fisicamente all'indice 0
    }

    #[test]
    #[should_panic(expected = "Index out of bounds!")]
    fn test_indexing_out_of_bounds() {
        let mut cb = CircularBuffer::new(5);
        cb.write(1).unwrap();
        cb.write(2).unwrap();

        // Ci sono solo 2 elementi (indici logici 0 e 1).
        // Provare ad accedere all'indice 2 DEVE scatenare un panic.
        // L'attributo #[should_panic] fa in modo che il test passi SOLO se il programma crasha qui!
        let _ = cb[2]; 
    }

    #[test]
    fn test_index_mut() {
        let mut cb = CircularBuffer::new(5);
        cb.write(10).unwrap();
        cb.write(20).unwrap();

        // Verifichiamo i valori iniziali
        assert_eq!(cb[0], 10);
        assert_eq!(cb[1], 20);

        // Usiamo IndexMut per modificare direttamente i valori sul posto!
        cb[0] = 99;
        cb[1] += 5; // Possiamo anche usare gli operatori composti!

        // Verifichiamo che la modifica abbia avuto successo
        assert_eq!(cb[0], 99);
        assert_eq!(cb[1], 25);
        
        // Verifichiamo che l'ordine di estrazione (read) rispetti le modifiche
        assert_eq!(cb.read(), Some(99));
        assert_eq!(cb.read(), Some(25));
    }

    #[test]
    fn test_deref_contiguous() {
        let mut cb = CircularBuffer::new(5);
        
        // Scriviamo 3 elementi (senza far fare il giro alla coda)
        cb.write(10).unwrap();
        cb.write(20).unwrap();
        cb.write(30).unwrap();

        // Sfruttiamo il Deref! 
        // In Rust, chiamare &cb in un contesto dove è richiesta una slice
        // invoca automaticamente deref().
        let slice: &[Option<i32>] = &cb;

        // Verifichiamo che la slice contenga esattamente i nostri 3 elementi
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0], Some(10));
        assert_eq!(slice[1], Some(20));
        assert_eq!(slice[2], Some(30));
    }

    #[test]
    #[should_panic(expected = "The circular buffer is not contiguous!")]
    fn test_deref_non_contiguous_panics() {
        let mut cb = CircularBuffer::new(3);
        
        // Riempiamo il buffer
        cb.write(1).unwrap();
        cb.write(2).unwrap();
        cb.write(3).unwrap();

        // Leggiamo un elemento (head diventa 1)
        cb.read();
        cb.read();

        // Scriviamo un nuovo elemento (la coda wrap-arounda a 0)
        cb.write(4).unwrap();

        // Ora i dati in memoria sono spezzati: [Some(4), Some(2), Some(3)]
        // Provare a prendere in prestito la slice farà esplodere il test (che è ciò che vogliamo)!
        let _slice: &[Option<i32>] = &cb; 
    }

    #[test]
    fn test_try_deref() {
        let mut cb = CircularBuffer::new(3);

        // 1. Inseriamo elementi in modo che siano contigui
        cb.write(10).unwrap();
        cb.write(20).unwrap();

        // try_deref() deve avere successo e restituire la slice
        let result_ok = cb.try_deref();
        assert!(result_ok.is_ok());
        let slice = result_ok.unwrap();
        assert_eq!(slice[0], Some(10));
        assert_eq!(slice[1], Some(20));

        // 2. Creiamo lo scenario "spezzato"
        cb.write(30).unwrap(); // Riempiamo (buffer: 10, 20, 30)
        cb.read();             // Togliamo 10 (spazio all'indice 0)
        cb.read();
        cb.write(40).unwrap(); // Wrap-around! Il 40 finisce all'indice 0

        // Ora i dati fisici sono [Some(40), Some(20), Some(30)]
        // try_deref() DEVE fallire restituendo il nostro errore personalizzato
        let result_err = cb.try_deref();
        assert_eq!(result_err, Err(CircularBufferError::NotContiguous));
    }
}