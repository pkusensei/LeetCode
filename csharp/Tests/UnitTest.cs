using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        List<IList<string>> paths = [["a"], ["c"], ["d"], ["a", "b"], ["c", "b"], ["d", "a"]];
        Assert.AreEqual("""[["d"],["d","a"]]""", sol.DeleteDuplicateFolder(paths).Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        List<IList<string>> paths = [["a", "b"], ["c", "d"], ["c"], ["a"]];
        Assert.AreEqual("""[["c"],["c","b"],["a"],["a","b"]]""", sol.DeleteDuplicateFolder(paths).Print());
    }

    [TestMethod]
    public void TestMethod3()
    {
        List<IList<string>> paths = [["a"], ["c"], ["d"], ["a", "b"], ["c", "b"], ["d", "a"]];
        Assert.AreEqual("""[["c"],["c","d"],["a"],["a","b"]]""", sol.DeleteDuplicateFolder(paths).Print());
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}