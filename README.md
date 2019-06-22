# td-one

Work in progress.

## TODO

- [x] FIX BUG where explode thing doesn't work.
- [x] Add names to actors.
  - [x] Task types that depend on a specific actor should compare both the index
    and the name to the target they're given. If the name doesn't match, then
    they need to pick a new AI routine because the index now refers to a
    different entity.
  - [x] Alternatively, if the name doesn't match they could look back through the
    actor list for something that does match that name, because chances are
    it's nearby.
  - [x] Figure out how to display some kind of player-understandable name for tasks
    where the task involves targeting another actor. Apparently since String
    isn't copy, we can't put a name in the ActorRef.
- [ ] Prevent actors from leaving the grid.
  - [ ] This could be done by adding actors to the outskirts of the grid to block
    movement.
  - [ ] This could also be done by just making each actor smart enough not to
    leave the grid.
- [ ] Fix the map generation to have "biomes" (or similar).
- [ ] Add new actor types.
  - [ ] Resources
  - [x] Buildings
- [ ] Add new task types.
  - [ ] Collect resources
  - [ ] Carry resource home
- [x] Add support for spawning new actors based on a task execution.
  - [x] Make sure to add a name to newly spawned actors.
- [x] Add support for deleting actors based on a task execution.
  - [x] One interesting thing to keep in mind here is that all actor references
    will have index off by one errors for each removal done when referencing
    back to the main actor array.
  - [x] Each actor has a unique ID which we should also store whenever targeting
    an actor, and compare that ID to the target when trying to find it. If you
    don't find your target at the given ID, you can step backwards until you
    find the right ID and then update the target index.
- [x] Refactor TaskCompletion code to use builder pattern.
- [ ] Lock selected actor on click.
- [x] Remove the test textbox.
- [ ] Spawn actors based on some actual gameplay or simulation scenario instead
  of a bunch of test actors.
- [x] Add an actual GUI mod for all static textboxes.
  - [ ] Add more information to the GUI.
- [ ] Make sure that resizing is handled well (either don't allow it, or pass
  updated height/width through to GUI)
- [ ] Sometimes we get into Task::execute from Actor::render_all with an
  index that is out of bounds??? I have no idea how this is possible. Added
  debugging information in case this happens again.
- [ ] Change square actors to have their x,y reference their center instead
  of their top-left, which should also fix sight radius thing.
- [ ] Add combat.
- [ ] Add animations.
- [ ] Convert this TODO list to GitHub issues.