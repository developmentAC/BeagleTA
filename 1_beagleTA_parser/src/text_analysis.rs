use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Write;

// Tokenizes input text by splitting it into words, normalizing to lowercase, trimming non-alphanumeric characters, and stemming the result.
pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| {
            word.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|word| !word.is_empty())
        .map(stem) // Apply stemming to each word
        .collect()
}
// get and return the root word of a word.
pub fn stem(word: String) -> String {
    let mut word = word;
    if word.ends_with("ing") && word.len() > 4 {
        word.truncate(word.len() - 3);
    } else if word.ends_with("ed") && word.len() > 3 {
        word.truncate(word.len() - 2);
    } else if word.ends_with("es") && word.len() > 3 {
        word.truncate(word.len() - 2);
    } else if word.ends_with('s') && word.len() > 2 {
        word.truncate(word.len() - 1);
    }
    word
}

// create a list of stop words for use
pub fn load_stop_words() -> HashSet<String> {
    // short list
    let stop_words = "a,an,the,and,or,but,if,then,else,when,at,by,for,with,about,against,between,to,from,in,out,on,off,over,under";
    // long list (you never know when you might need this!)
    // let stop_words = "i,me,my,myself,we,our,ours,ourselves,you,your,yours,yourself,yourselves,he,him,his,himself,she,her,hers,herself,it,its,itself,they,them,their,theirs,themselves,what,which,who,whom,this,that,these,those,am,is,are,was,were,be,been,being,have,has,had,having,do,does,did,doing,a,an,the,and,but,if,or,because,as,until,while,of,at,by,for,with,about,against,between,into,through,during,before,after,above,below,to,from,up,down,in,out,on,off,over,under,again,further,then,once,here,there,when,where,why,how,all,any,both,each,few,more,most,other,some,such,no,nor,not,only,own,same,so,than,too,very,s,t,can,will,just,don,should,now";
    stop_words.split(',').map(String::from).collect()
}

// remove stop words from text samples
pub fn remove_stop_words(text: &str, stop_words: &HashSet<String>) -> String {
    tokenize(text)
        .into_iter()
        .filter(|word| !stop_words.contains(word))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn calculate_cosine_similarity(counts: &[usize]) -> io::Result<f64> {
    let dot_product: usize = counts.iter().sum();
    let magnitude_a: f64 = (counts.iter().map(|&x| (x * x) as f64).sum::<f64>()).sqrt();
    let magnitude_b: f64 = (counts.len() as f64).sqrt();
    if magnitude_a > 0.0 && magnitude_b > 0.0 {
        Ok(dot_product as f64 / (magnitude_a * magnitude_b))
    } else {
        Ok(0.0)
    }
} // end of calculate_cosine_similarity()

// Function to write statistics to Markdown
pub fn write_statistics_to_markdown(
    keywords: &[String],
    heatmap_data: &[Vec<f64>],
    output_dir: &str,
) -> io::Result<()> {
    let stats_file_path = format!("{}/log.md", output_dir);
    // let mut file = fs::File::create(&stats_file_path)?;
    let mut file = File::create(&stats_file_path)?;
    writeln!(file, "# Data Statistics")?;
    for (i, keyword) in keywords.iter().enumerate() {
        let column: Vec<f64> = heatmap_data.iter().map(|row| row[i]).collect();
        let mean = column.iter().copied().sum::<f64>() / column.len() as f64;
        let max = column.iter().cloned().fold(f64::MIN, f64::max);
        let min = column.iter().cloned().fold(f64::MAX, f64::min);
        let std_dev =
            (column.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / column.len() as f64).sqrt();
        writeln!(
            file,
            "- **{}**: mean={:.2}, max={:.2}, min={:.2}, std_dev={:.2}",
            keyword, mean, max, min, std_dev
        )?;
    }
    Ok(())
} // end of write_statistics_to_markdown
