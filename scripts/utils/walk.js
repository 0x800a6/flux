const fs = require('fs');
const path = require('path');

let TMP_WALK_FILES = [];

function walk(dir, filter = null) {
    if (fs.statSync(dir).isDirectory()) {
        const files = fs.readdirSync(dir);
        files.forEach(file => {
            walk(path.join(dir, file), filter);
        });
    } else {
        if (filter && !dir.includes(filter)) {
            return;
        }
        TMP_WALK_FILES.push(dir);
    }

    return TMP_WALK_FILES;
}

module.exports = {
    walk
};