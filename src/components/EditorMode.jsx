import React from "react";

function EditorMode({ recording }) {
    return (
        <div className="w-full h-screen flex items-center justify-center">
            <div className="glass w-3/4 h-3/4 flex flex-col items-center justify-center p-6 rounded-lg">
                {recording ? (
                    <div className="text-center">
                        <h2 className="text-xl font-semibold mb-4">
                            Editing: {recording}
                        </h2>
                        <div className="w-full h-48 glass flex items-center justify-center rounded-lg">
                            <p className="text-gray-300">Video Editor Placeholder</p>
                        </div>
                    </div>
                ) : (
                    <p className="text-gray-400">No recording selected. Choose a recording to edit.</p>
                )}
            </div>
        </div>
    );
}

export default EditorMode;
