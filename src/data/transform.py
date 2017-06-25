import json


def pad(number_string, padding_char, padding_length):
    while len(number_string) < padding_length:
        number_string = padding_char + number_string
    return number_string


for ind in range(1, 7):
    with open("hsk-{ind}.json".format(ind=ind), mode='r') as opened_file:
        all_content = json.load(opened_file)
        words = all_content['words']
        max_num = len(words)
        max_num_length = len(str(max_num))

        new_words_structure = {}

        for word in words:
            new_words_structure[word['metadata']['id']] = word
            del new_words_structure[word['metadata']['id']]['metadata']['id']

        all_content['words'] = new_words_structure

        with open("hsk-{ind}-updated.json".format(ind=ind),
                  mode='w',
                  encoding='utf8') as write_file:
            write_file.write(
                json.dumps(
                    all_content,
                    indent=4,
                    ensure_ascii=False))
