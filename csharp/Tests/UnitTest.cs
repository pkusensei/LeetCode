using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("H2O", "H2O")]
    [DataRow("H2MgO2", "Mg(OH)2")]
    [DataRow("K4N2O14S4", "K4(ON(SO3)2)2")]
    public void TestMethod1(string exp, string s)
    {
        Assert.AreEqual(exp, sol.CountOfAtoms(s));
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}