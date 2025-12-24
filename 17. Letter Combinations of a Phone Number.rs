//17. Letter Combinations of a Phone Number

use std::collections::HashMap;

impl Solution {
    // digits: String -> Girdi olarak alınan rakamlardan oluşan string
    // dönüş tipi: Vec<String> -> olası kombinasyonların listesi
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];  // boş Vec<String>
        }

        // phone_map: HashMap<char, &str>
        // char: rakam karakteri ('2'..'9')
        // &str: o rakama karşılık gelen harflerin string dilimi
        let phone_map: HashMap<char, &str> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ].iter().cloned().collect();

        // result: Vec<String> -> oluşturulan tüm kombinasyonları tutar
        let mut result = Vec::new();

        // path: String -> şu anki kombinasyonu tutar, geçici değişken
        let mut path = String::new();

        // backtrack fonksiyonu parametreleri ve türleri:
        //
        // digits: &str -> rakamların tamamının string dilimi (referans)
        // index: usize -> şu anda hangi rakamda olduğumuzu gösteren indeks
        // path: &mut String -> şu anki kombinasyon yolu (mutable referans)
        // phone_map: &HashMap<char, &str> -> rakamlar-harfler haritası (referans)
        // result: &mut Vec<String> -> sonuç kombinasyonların mutable referansı
        fn backtrack(
            digits: &str,
            index: usize,
            path: &mut String,
            phone_map: &HashMap<char, &str>,
            result: &mut Vec<String>,
        ) {
            // Eğer kombinasyon uzunluğu input uzunluğuna eşitse
            if index == digits.len() {
                result.push(path.clone());  // şu anki kombinasyonu sonuçlara ekle
                return;
            }

            // Şu anki rakamın harflerini al
            if let Some(&letters) = phone_map.get(&digits.chars().nth(index).unwrap()) {
                // Harflerin her biri için:
                for ch in letters.chars() {
                    path.push(ch);  // harfi ekle
                    backtrack(digits, index + 1, path, phone_map, result);  // sonraki rakama geç
                    path.pop();  // geri dön (backtrack)
                }
            }
        }

        // Backtracking başlat
        backtrack(&digits, 0, &mut path, &phone_map, &mut result);
        result
    }
}
