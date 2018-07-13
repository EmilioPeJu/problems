#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

inline bool is_vowel(char a) {
	return a=='a' || a=='e' || a=='i' || a=='o' || a=='u';
}

int main() {
	ios::sync_with_stdio(false);
	string word;
	bool is_romaji = true;
	cin >> word;
	for(size_t i=1;i<word.size();i++) {
		if (!is_vowel(word[i-1]) && !is_vowel(word[i]) && word[i-1]!='n') {
			is_romaji = false;
			break;
		}
	}
	is_romaji = is_romaji && (word[word.size() - 1] == 'n' || is_vowel(word[word.size() - 1]));
	if (is_romaji)
		cout << "YES" << endl;
	else
		cout << "NO" << endl;
	return 0;
}
