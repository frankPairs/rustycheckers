fetch("./rustycheckers.wasm")
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, {
    env: {
      notify_placemoved(fx, fy, tx, ty) {
        console.log(`A piece moved from ${fx}-${fy} to ${tx}-${ty}`);
      },
      notify_placecrowned(x, y) {
        console.log(`A piece was crowned at ${x}-${y}`)
      }
    }
  }))
  .then(results => {
    const { get_piece, get_current_turn, move_piece } = results.instance.exports;

    console.log(`At start, current turn is ${get_current_turn()}`);

    const piece = get_piece(0, 7);
    console.log(`Piece at 0-7 is ${piece}`);

    const res = move_piece(0, 5, 1, 4);
    console.log(`First move result: ${res}`);
    console.log(`Turn after move: ${get_current_turn()}`);

    // Illegal move
    const bad = move_piece(1, 4, 2, 3);

    console.log(`Illegal move result: ${bad}`);
    console.log(`Turn after illegal move: ${bad}`);
  })
  .catch(console.error);