## General Idea
`Jurts` should be a program that makes it easy to brainstorm jurt construction ideas without relying on pen and paper, which, I argue, is slower and more error prone than a computer assisted design approach. The main benefits are, that you can show what you intend to do with others as well as (later) verify some static properties and generate construction plans. This might but does not have to include Mast and Abspann-pläne. This should enable the scouts community to plan even greater constructions, way beyond what is currently possible, and fully unlock the potential of the dynamic jurt-system.

It should only be possible to create well-formed jurts, because everything else would be nonsensical, regarding the aforementioned goals. We thus continue to create a mathematical model of jurts that inherits these properties and forbids ill-formed constructions.  We start with a 2D version which is later extended to 3D. This model can then be translated easily to a visualization in `bevy`.

## Notation
- A jurt is any possible tent from this list: `Kote`, `Jurte`, `Groß-Jurte`, `Gigagroß-Jurte` and `Theater-Jurte` even though `Kote` is not a jurt definitionally.
- We call jurt poles "vertices" and their connecting `sides` edges in the 2D case. For 3D, we still call poles single vertices even though they are defined by two actual vertices. This is because a 3D jurt is a primitive extrusion of the 2D one.
- A `crown` is the roof-ring of a jurt. It has a hole in the middle for the centered trunk.


## Construction of the Model
The construction will be (in the simple case) conducted in a topdown 2D canvas as that should be sufficient for fast brainstorming. A construction starts from a single jurt in the center of the view that can be extended iteratively with other jurts to form bigger constructions. A drawer should offer the basic jurt kinds from which the user can choose. When adding a new jurt, there are three ways to connect it to the existing construction:

1. Add it by overlapping single `side`. A `side` is defined as a single 165cm × 165cm plane (Seitenplane) and the two poles left and right to it.
2. Add it by overlapping only a single pole. Since the jurts are only connected by one point, they have one degree of freedome, regarding the angle between the two.
3. Adding it by overlapping _two_ `sides`. This is the usal way of overlapping jurts because it gives more space to move between them. When doing so, a bit of fabric overlaps between the two roofs, which has to be pulled upwards or layered on top of each other to create a rain-proof composite roof.

From the three described ways, 1. and 3. are focused for now with 3. being prioritised over the other.

## From Jurt to Construction
A construction is a composite of more than one jurt. We count a single jurt as a 0-composition and every extension increase the "count" of the construction by one to form a (n+1)-construction.

## Remark 1: Properties
To create a sensical model, some properties have to be fulfilled:

1. Roofs are not allowed to intersect any way besides the aforementioned primitive overlap when connecting across two sides. Everything else will create too much overhanging fabric and guy lines in nonsensical locations.
2. The area of a construction has to be a _primitive polygon_. This might be relaxed though.

Every construction (also a 0-construction) should provide some derived quantities:

3. Every construction should provide a notion of finding out whether a point lies within it.
3.a. similar for "too far in"
4. Every construction should provide a notion of finding possible extension points for generic but specific jurt.

## Adding along sides
When adding along sides, the existing construction (A) can compute the possible _center locations_ of the new jurt(B) simply by its (B)diamater and their own sides(A). Every jurt is approximated by a circle which intersects all the boundary poles of a jurt. The more vertices, the better an approximation of a circle is and whether the approximation is too groase for a Kothe has to be determined.

The simples approach to get _valid_ center positions is by iterating every `side` of a construction and acting as if the jurt was added there and adding the derived center to a list. Afer that, the list has to be checked again, since positioning the jurt on any of the centers might create a jurt that overlaps too much with the existing construction and thus violates Property 1.

A second run is thus conducted where the jurt is placed on every possible center c_i. c_i is then filtered out depending on whether it overlaps too much with the construction. This is done by checking whether any point of the "outer surface" of the construction (Property 3.) lies too far inside the new jurt (Property 5.a).

Whats left is a list of valid center positions that can be compared to the mouse position to find the closest one. By clicking, the action of adding a jurt is initiated.

# Different models
We now create four consecutive models of a construction that build (ha-ha) upon each other to finally show how adding a new jurt can be done.

## From 2D to 3D
The first models is the basic 2D version of a construction plan, which lies flat in 2D space and consists of simple polygons. To get to the 3D-model, the underlying 2D-polygon is extruded perpendicular to the third dimension to form a _prism_. This forms the simple "base" of a jurt. Also, roofs (crowns) even though existent in 2D-theoretically, only get a meaning in 3D. In a 3D construction it should thus be possible to index by one more parameter $j$, expressing the distance from the ground to account for the new dimension. 
The third dimension also adds the possibility to stack jurts on top of each other under the restriction that there are no floating jurts.

Possible heights should be:
1. (1) Floor-level
2. (2) 165cm (jurt)
3. (3) 200cm (s-jurt)
4. (4) 265cm (jurt with roof)
5. (5) 300cm (s-jurt with roof)
6. (6) …

… continuing to some arbitrary height in the shown manner.

## From 3D to complex
In the 3D-Model, single jurts form convex polygons and constructions primitive polygons, giving us nice properties for the needed computations. This is an over-simplification of the reality though, disregarding the inner structure of such a construction. Since it is interesting to keep a notion of sub-jurts as they will be addressed regularly while editing, we need to extend the 3D-Model by filling in all the points that "lie inside the construction". In the simplest form, we can do so by set union of the vertices and edges, effectively putting both constructions into "the same world". The vertices and edges then form complex meshes that overlap in many places.

Since we strive for a good representation, and thus clear alignment of jurts, we need to properly glue together the two constructions instead of letting them live besides each other. To do so, we effectively need to merge vertices that "lie close to each other" into shared vertices. We call this the "trimmed" version of a construction. Also, if roofs overlap, we could decide to add _new_ vertices to accound for the high-strapping of roofs. When adding a new jurt, this is exactly what needs to happen.

Remark: At some point, flying jurt roofs (flysheet) should be added.

## From Complex to Visual
The visual model is a straight forward translation from the Complex model to blender primitives (planes and rods effectively). Since it is a straightforward 1:1 translation, it is not discussed any further.


## Removing Jurts
When removing a single jurt from a collection, the "merged vertices" have to be re-added.


# A discrete implementation

## Implementation Properties
1. It should create an "outer surface" meaning a distiction between whats "inside" and "outside".
2. The set V of outer points is ranged over by I, forming vertices v_i, should form a _circular order_. i.e. $v_1 < v_2 < … < v_n < v_1$. Where "<" is a relation stating that the first operand "is left of, or before" the second. This way one can effectively "walk around" the jurt by increasing the index and complete a cicle when every vertex (pole) was visited exactly once. 

We now try to lay out the general structure of the implementation.

A theoretical jurt should be a datatype that holds at least two vectors. One for vertices and one for edges to form the mesh. It sould also hold the center position of the jurt and well as the number of edges. Possibilites are 8, 12, 16, and 18. Edges should hold the information, whether there is a plane or not so we put it into a struct.

```rust
struct Edge {
  start: usize, // indexes into vertices
  occupied: bool,
}

struct Jurt2d<sides: usize> {
  edges: [Edge; sides-1],
  vertices: [Vec2; sides],
  center: Vec2,
  radius: f32, // added to not have to recompute
}

trait Visualizable {
  // creates a bevy instance of the jurt
  fn instatiate();
}

trait Construction: Index<usize> {
  fn lies_inside(&self, pos: Vec2);
  fn index(&self, pos: usize);
  fn centers(&self) -> Vec<Vec2>;
}

impl Construction for Jurt2d {
  fn lies_inside(&self, pos: Vec2) {
    self.center.distance(pos) > self.radius 
  }

  fn index(&self, pos: usize) {
    &self.vertices[i]
  }
}

```

We then extend the definition of a jurt to a construction:
This construction collects all the sub-components such that we

```rust
struct Construction {
  parts: Vec<dyn Jurt>,
  vertices: Vec<Vec2>,
  hull: Vec<Edge>
}

impl Construction {
  /// Removes a single component from the construction
  /// "Repairs" the jurt where simplifications happened
  
  fn remove_component(id: usize) {
    
  }
}

```




