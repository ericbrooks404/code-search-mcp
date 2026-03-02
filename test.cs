using System;

namespace TestNamespace
{
    public class GameCharacter
    {
        public string Name { get; set; }
        public int Health { get; set; }

        public GameCharacter(string name, int health)
        {
            Name = name;
            Health = health;
        }

        public void TakeDamage(int damage)
        {
            Health -= damage;
            if (Health < 0)
            {
                Health = 0;
            }
        }

        public bool IsAlive()
        {
            return Health > 0;
        }
    }

    public interface ISpellCaster
    {
        void CastSpell(string spellName);
        int GetMana();
    }

    public enum CharacterClass
    {
        Warrior,
        Mage,
        Rogue
    }
}
