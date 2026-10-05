#include <bits/stdc++.h>
using namespace std;


//the allowed characters will be lowercase a-z. aswell as . and , amd ' '
struct Ahocorasick{
    vector<vector<int>> adj;
    vector<int> mxsuff;
    vector<string> vertflag;
    int state = 0;
    int getindex(char c){
        if('a' <= c <= 'z') return c - 'a';
        if(c == '.') return 26;
        if(c == ',') return 27;
        if(c == ' ') return 28;
    }
    char getchar(int i){
        if(i<26) return 'a' + i;
        if(i == 26) return '.';
        if(i == 27) return ',';
        if(i == 28) return ' ';
    }
    int create_new_node(){
        int i = adj.size();
        adj.push_back(vector<int>(29,-1));
        mxsuff.push_back(0);
        vertflag.push_back("");
    }
    void insert_word(string keyword){
        int state = 0;
        for(auto c : keyword){
            if(adj[state][getindex(c)] == -1){
                state = adj[state][getindex(c)] = create_new_node();
            }
            else{
                state = adj[state][getindex(c)];
            }
        }
        vertflag[state] = keyword;
    }
    int getnext(int state, char c){
        int ci = getindex(c);
        while(adj[state][ci] == -1 && state != 0) state = mxsuff[state];
        if(adj[state][ci] != -1) return adj[state][ci];
        else return 0;
    }
    int getmxsuff(string nodestr){
        int state = 0;
        for(int i = 1; i< nodestr.size(); i++){
            state = getnext(state, nodestr[i]);
        }
        return state;
    }
    Ahocorasick(vector<string> keywords){
        create_new_node();
        adj[0][getindex(' ')] = create_new_node();
        for(auto x : keywords) insert_word(x);
        queue<pair<int,string>> daque;
        daque.push({0,""});
        while(!daque.empty()){
            pair<int,string> cur = daque.front();
            daque.emplace();
            mxsuff[cur.first] = getmxsuff(cur.second);
            for(int i = 0; i< adj.size(); i++){
                if(adj[cur.first][i] == -1) continue;
                daque.push({adj[cur.first][i], cur.second + getchar(i)});
            }
        }
    }
    Ahocorasick(){}
    string make_step(char c){
        state = getnext(state,c);
        return vertflag[state];
    }
    void reset(){
        state = 0;
    }
};