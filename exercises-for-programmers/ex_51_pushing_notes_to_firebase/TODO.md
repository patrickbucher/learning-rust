    curl -X PUT \
        -d '{"foo": "bar"}' \
        https://ex-51-notes-default-rtdb.europe-west1.firebasedatabase.app/foo.json

    curl -X GET \
        https://ex-51-notes-default-rtdb.europe-west1.firebasedatabase.app/foo.json
