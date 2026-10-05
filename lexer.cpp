#include<bits/stdc++.h>
#include<aho-corasick.h>
using namespace std;
struct Text{
    string contents;
    Text(string s) : contents(s){}
};

using token = variant<Text>;
struct Tokenizer{
    Ahocorasick corasick;
    map<string,token> keywords;
    Tokenizer(map<string,token> keywords):keywords(keywords){
        vector<string> token_strings;
        for (auto [str, tok] : keywords){
            token_strings.push_back(str);
        }
        corasick = Ahocorasick(token_strings);
    }
    vector<token> parse(string s){
        corasick.reset();
        vector<token> ret;
        vector<char> cur;
        for(auto c : s){
            cur.push_back(c);
            string wrd = corasick.make_step(c);
            if(wrd == "") continue;
            for(int i = 0; i< min(wrd.size(),cur.size()); i++) cur.pop_back();
            if(cur.size() != 0) ret.push_back(Text(string(cur.begin(),cur.end())));
            cur.clear();
            ret.push_back(keywords[wrd]);
        }
        if(cur.size() != 0) ret.push_back(Text(string(cur.begin(),cur.end())));
        return ret;
    }
};